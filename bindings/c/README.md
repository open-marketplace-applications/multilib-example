
# multilib-example C lib

> This is an experimental C binding to multilib-example.

## Setup
Follow these steps to setup the project.

### 1. Get the Code.
Clone repo, change branch and go to the right directory.
```bash
git clone https://github.com/open-marketplace-applications/multilib-example
cd multilib-example/bindings/c
```

### 2. Build it
```bash
make
```

### 2. Run it
```bash
./main
```

## Development

### generate new header file

While development, you may need to generate a new header file `identity.h`.

It uses [cbindgen](https://github.com/eqrion/cbindgen) to creates C/C++11 headers for Rust libraries which expose a public C API.

To install cbindgen, you just need to run

```bash
cargo install --force cbindgen
```

After you successful installed cbindgen, run this:
```bash
cbindgen --config cbindgen.toml --crate multilib-example-c --output multilib-example.h
```

This produces a header file for C++. For C, add the --lang c switch.
cbindgen also supports generation of Cython bindings, use --lang [cython](https://cython.org/) for that.