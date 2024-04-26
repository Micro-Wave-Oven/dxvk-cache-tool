use sha1::Sha1;

pub const HASH_SIZE: usize = 20;
pub const MAGIC_STRING: [u8; 4] = *b"DXVK";


// Header Struct
pub struct DxvkStateCacheHeader {
    pub magic:      [u8; 4],
    pub version:    u32,
    pub entry_size: u32
}


// Entry Structs
//  The cache entry header (not the cache header) has the following format:
//   entryType -> 1 bit
//   stageMask -> 5 bits
//   entrySize -> 26 bits
pub struct DxvkStateCacheEntryHeader {
    pub raw: u32
}

impl DxvkStateCacheEntryHeader {
    pub fn entry_size(&self) -> usize {
        let raw_bytes: [u8; 4] = self.raw.to_le_bytes();
        let size: usize = ((raw_bytes[3] as u32) << (2 + 16) | (raw_bytes[2] as u32) << (2 + 8) | (raw_bytes[1] as u32) << 2 | (raw_bytes[0] as u32) >> 6) as usize;
        size
    }
}


pub struct DxvkStateCacheEntry {
    pub header: DxvkStateCacheEntryHeader,
    pub hash:   [u8; HASH_SIZE],
    pub data:   Vec<u8>
}

impl DxvkStateCacheEntry {

    //noinspection ALL
    pub fn with_header(header: DxvkStateCacheEntryHeader) -> Self {
        DxvkStateCacheEntry {
            data:   vec![0; header.entry_size() as usize],
            hash:   [0; HASH_SIZE],
            header: header,
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut hasher = Sha1::default();
        hasher.update(&self.data);
        let hash = hasher.digest().bytes();

        hash == self.hash
    }
}
