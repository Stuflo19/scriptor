# scriptor

### Running the application:

Ensure rust is installed following: 
```Bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Run using the command:
```Bash
cargo run
```

Build release version using:
```Bash
cargo build --release
```

To test using the release version you can run:
```Bash
ln -s "$PWD/target/release/scriptor" /usr/bin
```

