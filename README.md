# Rust `wc` Command Implementation

This is a simple implementation of the Unix `wc` command in Rust. The `wc` command is used to count the number of lines, words, and bytes in a file.

## Features

- Counts the number of lines, words, and bytes in a file.
- Supports flags to specify which counts to display (`-l` for lines, `-w` for words, `-c` for bytes).
- Handles both file names provided as arguments and standard input.

## Installation

You can clone this repository and build the project using Cargo, Rust's package manager and build system:

```
git clone https://github.com/Wesley-Arizio/rwc.git
cd rwc
cargo build --release
```

## Usage

Run the compiled binary with the file name as an argument to count lines, words, and bytes in the file:

```
./target/release/rwc file.txt
```

You can also use flags to specify which counts to display:

```
./target/release/rwc -l file.txt  # Count lines only
./target/release/rwc -w file.txt  # Count words only
./target/release/rwc -c file.txt  # Count bytes only
```

If no flags are provided, all counts will be displayed by default.

## Example

```
$ ./target/release/rwc test.txt
Lines: 7145
Words: 58164
Bytes: 339292
```