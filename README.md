# Address Generator

A command-line tool for generating cryptocurrency addresses and BIP-39 mnemonic phrases. Supports Ethereum Virtual Machine (EVM) addresses, Solana addresses, and 12-word mnemonic phrases (256-bit entropy).

## Features

- Generate EVM-compatible addresses with public and private keys.
- Generate Solana addresses with public and private keys.
- Generate BIP-39 compliant 12-word mnemonic phrases in English.
- Save generated data to CSV files or display in the console.
- Configurable via command-line arguments for batch generation.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.82.0 or higher recommended)
- Cargo (included with Rust)

### Steps

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/19-xiaogao/Address-Generator.git
   cd Address-Generator
   ```

2. **Build the Project**:
   ```bash
   cargo build --release
   ```

3. **Run the Tool**:
   ```bash
   cargo run --release -- [OPTIONS]
   ```

## Usage

The tool is controlled via command-line arguments. Run the following to view available options:

```bash
cargo run -- --help
```

### Command-Line Options

| Option                | Description                                   | Example            |
|-----------------------|-----------------------------------------------|--------------------|
| `-c, --count <N>`     | Number of addresses/mnemonics to generate     | `-c 5`            |
| `-n, --network <NETWORK>` | Network type: `evm`, `solana`, or `mnemonic` | `-n mnemonic`     |
| `-o, --output <PATH>` | Save output to a CSV file                    | `-o output.csv`   |
| `--version`           | Display version and exit                     | `--version`       |

### Examples

1. **Generate 2 Mnemonic Phrases**:
   ```bash
   cargo run -- -c 2 -n mnemonic
   ```
   **Output**:
   ```
   🚀 Generating 2 MNEMONIC entries...
   Index: 1, Mnemonic: apple banana cherry date elder fig grape honey ink jet kite lemon mango nectar orange pear quince rose seed tree umbrella violet wolf xray
   Index: 2, Mnemonic: zebra yellow xylem whale violet unicorn tiger snake rose quince pear orange nectar mango lemon kite jet ink honey grape fig elder date cherry
   ✅ Generation completed
   ```

2. **Generate 3 EVM Addresses**:
   ```bash
   cargo run -- -c 3 -n evm
   ```
   **Output**:
   ```
   🚀 Generating 3 EVM entries...
   Index: 1, Public Address: 0x123..., Private Key: 456...
   Index: 2, Public Address: 0x789..., Private Key: abc...
   Index: 3, Public Address: 0xdef..., Private Key: 123...
   ✅ Generation completed
   ```

3. **Generate 2 Solana Addresses and Save to CSV**:
   ```bash
   cargo run -- -c 2 -n solana -o solana_addresses.csv
   ```
   **Output**:
   ```
   🚀 Generating 2 SOLANA entries...
   ✅ Generation completed, saved to solana_addresses.csv
   ```
   **CSV Content**:
   ```
   Index,Network,Public Address,Private Key
   1,solana,5aB...,xyz...
   2,solana,7cD...,pqr...
   ```

4. **Display Version**:
   ```bash
   cargo run -- --version
   ```
   **Output**:
   ```
   v0.1.0
   ```

## Dependencies

The project relies on the following Rust crates:

- `clap = { version = "4.5", features = ["derive"] }`: Command-line argument parsing.
- `ethers = "2.0"`: EVM address generation.
- `solana-sdk = "2.1"`: Solana address generation.
- `bip39 = "2.1"`: BIP-39 mnemonic phrase generation.
- `rand = "0.8"`: Random entropy generation.
- `csv = "1.3"`: CSV file output.
- `anyhow = "1.0"`: Error handling.

See `Cargo.toml` for the full dependency list.

## Security Notes

- **Mnemonic Phrases**: Mnemonic phrases are seeds for private keys. Never share them publicly. The tool saves mnemonics in plain text when outputting to CSV. For production use, consider encrypting the output or avoiding file storage.
- **Private Keys**: EVM and Solana private keys are sensitive. Handle CSV outputs securely.
- **Randomness**: The tool uses `OsRng` for cryptographically secure random number generation, suitable for generating secure keys and mnemonics.

## Limitations

- Currently supports only 24-word mnemonic phrases (256-bit entropy). Support for other lengths (12, 15, 18, 21 words) can be added.
- Mnemonic phrases are not used to derive EVM or Solana addresses. To add this feature, consider integrating BIP-44 derivation paths.
- No encryption for CSV outputs. Use with caution in production environments.

## Contributing

Contributions are welcome! Please submit issues or pull requests to the [GitHub repository](https://github.com/19-xiaogao/Address-Generator).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
