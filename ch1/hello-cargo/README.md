Cargo is Rust's build system and package manager.
Something like pip and setuptools combined for python, and npm for JS.

Vast majority of rust projects use Cargo to manage dependencies.

To create a new rust project with cargo, simply type ```$ cargo new``` to the command line. Then Cargo will configure empty rust project.

Cargo configures the project with a file named Cargo.toml. Cargo.toml contains basic metadata information of the project such as name, version or edtions and so on in addition to package dependencies.

Cargo expects the project source codes to be in src layout, which means that all the codes should be under src/ directory. There should be only misc files such as README or LISENCE at the project top level.

To build a project, you can run ```$ cargo build``` to just build the executable binary without running or ```$ cargo run``` to build and execute the built result instantly. ```$ cargo build --release``` will run extra optimizations to generate optimized binary for production.

Or, you can just check that your projects are in a 'compile-able state' by running ```$ cargo check``` this will quickly check your code to make sure it compiles without producing an executable. 
