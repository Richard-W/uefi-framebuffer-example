.PHONY: all
all:
	cargo xbuild --target x86_64-unknown-uefi

.PHONY: run
run: all
	uefi-run ./target/x86_64-unknown-uefi/debug/uefi-framebuffer-example.efi
