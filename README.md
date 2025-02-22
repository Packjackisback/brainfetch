# Brainfetch

Brainfetch is a fun little Brainfuck-like library written in Rust with support for API fetching and a few other features that help with handling changing apis.

## Installation

You can install from this repository by cloning the repo and building with `cargo build`.
Alternatively, you can use this library by adding brainfetch_lib to your Cargo.toml.

## Usage

```rust
use brainfetch_lib::BrainFetchInterpreter;
#[tokio::main]
async fn main() {
    let program = "+++++++[>+++++<-]>.";
    let mut interpreter = BrainFetchInterpreter::new(program);
    let s: String = interpreter.run().await;
    println!("{}", s);
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
