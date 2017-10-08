xed-sys
---

Automatically generated bindings via [`bindgen`](https://crates.io/crates/bindgen) for 
[Intel XED](https://software.intel.com/en-us/articles/xed-x86-encoder-decoder-software-library)


### Build Tools

* `libclang`
* `clang` and `clang++`
* `yasm`
* GNU/Linux based OS (please note only x64 targets are tested)
* `mkdir`
* `rustc`/`cargo`
* `git` (new enough to support submodules)
* `python v2.7` (to build xed)

You'll need `clang`, `clang++`, `ld`, and `ar` to be located in
`/usr/bin/`

### Building

`cargo build`

### License

This project should be considered to be under the Apache2 License

### Status

Currently this just includes the decode information
