# rust
Repo for learning rust. Following The Rust Programming Language book: https://doc.rust-lang.org/book/


Bookmark: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

## Installing

`rustup` is a utility for installing and managing rust versions.

Install with:

        curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

Update to the latest version:

        rustup update       // Update rust to latest version

`rustc --version`

## Tools

### Formatting
`rustfmt` is included with rustc. This will format the files to a standard.

### Build
`cargo` also included if you use rustup to install it. 

        cargo new hello-cargo       // Create a new project directory called hello-cargo

TOML is Cargo's config format
Packages would get added to the `[dependancies]` section heading. In Rust, these are called crates

`cargo build` will build the project and output the binary in ./target/debug/hello-cargo

`cargo run` to compile and run the code in one command.

`cargo check` checks the code compiles but doesn't create the binary

`cargo build --release` will compile the code with optimisations. It takes longer to compile but running the executable will run faster. It'll create the binary in ./target/release

`cargo update` will ignore the cargo.lock file and get the latest specifications which satisfy the constraints in cargo.toml

`cargo doc --open` will open docs for your dependencies and open them locally in a browser
