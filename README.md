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

## Installation using pre-built binaries

Got to the [release page](https://github.com/rubeniskov/audible-scrapper/releases) and download the latest release for your operating system.

## Install from Source

Clone the repository and install dependencies using `cargo`:

### Prerequisites

1. **Rust**: Ensure that Rust is installed on your system. You can install it by following the instructions on [rust-lang.org](https://www.rust-lang.org/).
2. **Cargo**: Cargo should be available as part of Rust for managing dependencies and building the project.

```bash
git clone https://github.com/rubeniskov/audible-scrapper.git
cd audible-scrapper-main
cargo install --path .
```

## Usage

Below is a basic usage example of the CLI, allowing you to specify output format and optionally filter by narrator.

```bash
audible-scraper --format <json|csv|jsonl|toml> [--narrator <narrator_name>]
CLI to scrape Audible audiobooks

Usage: audible-scrapper.exe [OPTIONS]

Options:
  -n, --narrator <NARRATOR>  Narrator name to filter audiobooks (optional)
  -k, --keywords <KEYWORDS>
  -f, --format <FORMAT>      Output format: jsonl, csv, json, or toml [default: json] [possible values: jsonl, csv, json, toml]
  -h, --help                 Print help
```

```bash
audible-scrapper --narrator "Jordi Salas"
[
  {
    "title": "1793 (Spanish Edition)",
    "narrator": "Jordi Salas",
    "language": "Español (Castellano)",
    "releaseDate": "2020-07-30",
    "sampleUrl": "https://samples.audible.com/bk/rhsp/002067/bk_rhsp_002067_sample.mp3"
  },
  ...
  {
    "title": "Una Visión Estratégica para los Retos Económicos del Siglo XXI (Narración en Castellano)",
    "narrator": "Jordi Salas",
    "language": "Español (Castellano)",
    "releaseDate": "2020-11-19",
    "sampleUrl": "https://samples.audible.com/bk/adbl/057704/bk_adbl_057704_sample.mp3"
  }
]
```

## Command line Arguments

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
