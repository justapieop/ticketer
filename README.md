# ticketer

CLI tools written in Rust to manage tickets

# Commands

Create a ticket

```bash
create (title) (subject)
```

List

```bash
list
```

Get a ticket

```bash
get (id)
```

Edit a ticket

```bash
edit (id)
```

Exit the app


```bash
exit
```

# Build instruction

You will need the rust toolchain for this. Download [here](https://rustup.rs/)

Before any builds, run tests:

```bash
cargo clippy -- -D warnings
cargo test --verbose
```

Build the project

```bash
cargo build --release
```