# Scriptor

<p align="center">
    <img src="https://github.com/Stuflo19/scriptor/actions/workflows/ci.yml/badge.svg" alt="Build status badge"/>
</p>

### Screenshots:
<img width="660" alt="Screenshot 2024-11-28 at 23 38 44" src="https://github.com/user-attachments/assets/9af5361c-b49a-42ae-ae62-bab3f2e23606">

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

