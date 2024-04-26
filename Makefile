VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")
RUST_VERSION = $(word 2, $(shell rustc -V))
LONG_VERSION = "$(VERSION) ( rev: $(GIT_REVISION), rustc: $(RUST_VERSION), build at: $(BUILD_TIME) )"
BIN_NAME = procs

export LONG_VERSION

.PHONY: clean release_win


clean:
	cargo clean

release_win:
	cargo build --locked --release --target=x86_64-pc-windows-msvc
	mv -v target/x86_64-pc-windows-msvc/release/${BIN_NAME}.exe ./
	7z a ${BIN_NAME}-v${VERSION}-x86_64-windows.zip ${BIN_NAME}.exe