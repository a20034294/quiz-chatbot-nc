# quiz-chatbot-nc with Rust

## Dependency
Ubuntu 18.04
```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    sudo apt update
    sudo apt install cargo
    sudo apt install pkg-config
    sudo apt install libssl-dev
```

## Usage
```bash
cd quiz-chatbot-nc
cp src/config-template.rs src/config.rs
vim src/config.rs # Edit configs
cargo run
```