# rust-demo
demo Rust programs

# Notes

- Install Rust

```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

- New project

```
cargo new --bin my_project
cd my_project
```

- Compile project

```
cargo build
```

- Compile & Run

```
cargo run
```

- Run

```
target/debug/my_project
```

- Compile & run `.rs` file directly without `cargo`

```
rustc src/main.rs
./main
```

- ignore `target` dir from git

```
echo '*target/*' >> .gitignore
```

- add Rust language support to Atom; https://github.com/zargony/atom-language-rust (install from within Atom preferences)

# Resources

https://doc.rust-lang.org/book/second-edition/

https://github.com/rust-lang/book

https://github.com/rust-lang/rust-by-example/tree/master/src

https://github.com/PistonDevelopers/piston
