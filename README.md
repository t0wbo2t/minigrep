# MiniGrep

A minimal command-line search tool written in Rust.
This project demonstrates basic CLI argument parsing, file handling, error management, environment variables, iterators, and unit testing.

## Project Structure

```text
.
├── Cargo.lock
├── Cargo.toml
├── output.txt
├── poem.txt
└── src
    ├── lib.rs
    └── main.rs
```

## Features

* Search for matching lines in a text file
* Case-sensitive search
* Optional case-insensitive search using an environment variable
* Unit tests for search functionality
* Proper error handling with `Result`

## Build

```bash
cargo build
```

## Run

### Case-sensitive search

```bash
cargo run -- rust poem.txt
```

### Case-insensitive search

Linux/macOS:

```bash
IGNORE_CASE=1 cargo run -- rust poem.txt
```

Windows PowerShell:

```powershell
$env:IGNORE_CASE=1
cargo run -- rust poem.txt
```

## Example Output

```text
Searching for pattern: 'rust' in file: 'poem.txt'.
Rust:
Trust me.
```

## Run Tests

```bash
cargo test
```

## Notes

* `search()` performs case-sensitive matching.
* `case_ignore_search()` performs case-insensitive matching.
* Invalid or missing command-line arguments are handled gracefully.
* `std::env::args()` expects valid Unicode input. Use `args_os()` if non-Unicode arguments must be supported.
