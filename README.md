# Keypair Recover

Convert a Base58 Solana private key into an `id.json` keypair file.

I created this tool because some wallets don't make it easy to export a keypair in the `id.json` format that Solana tools expect.
It converts a private key you provide into that format.

## Usage

```bash
cargo run
```

Choose whether to enter the Base58 string directly or provide a file containing it. The program prints the absolute path of the generated `id.json` file.
Note : This tool is useful when we export a private key from Phantom app which is in base58 formats.

Example prompt and output (the private key is hidden while you type):

```text
Choose 1, 2, or 3: 1
Enter the Base58 string (input hidden): [hidden]
Keypair saved to /path/to/id.json
Public key: <your public key>
```
