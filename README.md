# solana-key-convert

Convert solana private key format

## Install

```sh
cargo install --git https://github.com/Akagi201/solana-key-convert.git
```

## Usage

```sh
> solana-key-convert -j "[215, 1, 171, 175, 231, 181, 179, 120, 224, 189, 48, 90, 13, 158, 13, 165, 220, 191, 135, 139, 194, 233, 91, 224, 8, 60, 218, 42, 91, 130, 69, 64, 13, 3, 18, 42, 126, 82, 80, 116, 35, 226, 223, 12, 241, 214, 131, 35, 172, 53, 186, 123, 233, 68, 138, 72, 32, 120, 68, 126, 228, 28, 167, 235]"

5JKivoYFrKQGCau45wwn5zbgdMcrP7XPiu6yo2iYC55RmwxYMjJmqJzuUqXTsrWwRUBsefUShmR7GDXB5LwnQf3x
```

```sh
> solana-key-convert -b "5JKivoYFrKQGCau45wwn5zbgdMcrP7XPiu6yo2iYC55RmwxYMjJmqJzuUqXTsrWwRUBsefUShmR7GDXB5LwnQf3x"
[215, 1, 171, 175, 231, 181, 179, 120, 224, 189, 48, 90, 13, 158, 13, 165, 220, 191, 135, 139, 194, 233, 91, 224, 8, 60, 218, 42, 91, 130, 69, 64, 13, 3, 18, 42, 126, 82, 80, 116, 35, 226, 223, 12, 241, 214, 131, 35, 172, 53, 186, 123, 233, 68, 138, 72, 32, 120, 68, 126, 228, 28, 167, 235]
```
