# Audible Scraper

_Audible Scraper_ is a command-line tool (CLI) written in Rust that allows you to scrape information about audiobooks from Audible with various filters and output format options. It leverages internal modules to build customized queries and serialize results into formats like JSON, CSV, JSONL, and TOML.

## Features

- Filter audiobooks by narrator (optional).
- Export data in various formats, including:
  - **JSON**
  - **CSV**
  - **JSONL**
  - **TOML**
- Asynchronous implementation for scalability and efficiency.

## Prerequisites

1. **Rust**: Ensure that Rust is installed on your system. You can install it by following the instructions on [rust-lang.org](https://www.rust-lang.org/).
2. **Cargo**: Cargo should be available as part of Rust for managing dependencies and building the project.

## Installation

Clone the repository and install dependencies using `cargo`:

```bash
git clone <repository-url>
cd audible-scrapper-main
cargo build --release
```

## Usage

Below is a basic usage example of the CLI, allowing you to specify output format and optionally filter by narrator.

```bash
./target/release/audible_scraper --format <json|csv|jsonl|toml> [--narrator <narrator_name>]
```

### Usage Examples

1. **Extract in JSON format without narrator filter**:

   ```bash
   ./target/release/audible_scraper --format json
   ```

2. **Extract in CSV format filtered by narrator**:

   ```bash
   ./target/release/audible_scraper --format csv --narrator "Stephen Fry"
   ```

3. **Extract in TOML format**:

   ```bash
   ./target/release/audible_scraper --format toml
   ```

### Command line Arguments

- `--format`: Defines the output format. Available options: `json`, `csv`, `jsonl`, `toml`. **Default**: `json`.
- `--narrator`: Filters results to include only audiobooks narrated by the specified name. **Optional**.
- `--keywords`: Filters results to include only audiobooks with the specified keywords. **Optional**.

## Project Structure

- **`src/main.rs`**: Entry point of the CLI application, defining argument handling and execution logic.
- **`src/audio_book.rs`**: Defines the `AudioBook` struct representing audiobook data.
- **`src/builder.rs`**: Contains the construction of custom queries.
- **`src/params.rs`**: Defines the query parameters.
- **`src/scrapper.rs`**: Implements the main scraping logic for Audible.

## Common Error Handling

- **TOML Format Error**: If you encounter an `UnsupportedType(None)` error, ensure all fields in the `AudioBook` struct have simple types. Alternatively, use an intermediate data structure to serialize into `TOML`.

## Contributions

If you wish to contribute to this project:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/AmazingFeature`).
3. Submit a pull request.
