# comp-rs

[portuguese](README.pt-BR.md) | [english](README.md)

Compiler developed in [Rust](https://www.rust-lang.org) for the Compilers subject at Federal University of ABC, given by professor [Francisco Isidro Massetto](http://professor.ufabc.edu.br/~francisco.massetto/) in the second quarter of 2019.

This project was developed on `rustc 1.35.0 (3c235d560 2019-05-20)`

# Objective

Create a transpiler for a fictional language to C language by implementing the features described in [milestone v1.0](https://github.com/gmurayama/comp-rs/milestone/1).

# Install

To install Rust, the recommended way is using [rustup](https://www.rust-lang.org/tools/install), a toolchain installer that enables you to switch between different version of the compiler.

Linux or another Unix-like OS:

```bash
curl https://sh.rustup.rs -sSf | sh
```

# Compiling a program

There is some files in [examples](examples) folder showing the language basic syntax.

```rust
cargo run <path_to_file>
```

The transpiled source code will be printed at the scren and it will be saved a file named `out.c`.

# License

This project is under the MIT License - see the [LICENSE](LICENSE) file for details.