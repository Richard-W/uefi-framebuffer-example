uefi-framebuffer-example
========================

Dependencies
------------

### cargo-xbuild

```bash
$ cargo install cargo-xbuild
```

### lld-link

The `lld-link` application needs to be in `$PATH`. Ubuntu provides the `lld` package but it contains an executable named `lld-link-6.0`.

To make it work create a symlink in `/usr/local/bin`:

```bash
$ ln -s /usr/bin/lld-link-6.0 /usr/local/bin/lld-link
```

### Emulator

Install `qemu-system-x86_64` and `ovmf` packages on your system.

`uefi-run` is optional but a makes things a lot easier.

```bash
$ cargo install uefi-run
```

Building & Running
------------------

To build just issue

```bash
$ make
```

If `uefi-run` is installed you can start the application with

```bash
$ make run
```
