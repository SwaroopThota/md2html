# MD to HTML Converter

A lightweight Rust command-line utility that converts a Markdown file into a standalone HTML document.

## Project overview

This project reads a Markdown source file, parses a small subset of Markdown syntax, and writes a generated HTML file with a default stylesheet.

It is designed for quick local conversion of simple Markdown content into HTML without requiring external libraries or a web server.

## Features

- Converts Markdown headings (`#` through `######`)
- Converts unordered lists (`- item`)
- Converts blockquotes (`> text`)
- Converts horizontal rules (`---`)
- Supports fenced code blocks (```)
- Creates a standalone HTML file beside the source file
- Includes a default embedded CSS stylesheet

## File structure

```text
md2html_converter/
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── sample.md
├── src/
│   ├── file_helper.rs
│   ├── main.rs
│   └── md_to_html_converter.rs
└── target/
```

### Source files

- `src/main.rs` — CLI entry point. Reads the input file path provided by the user and triggers conversion.
- `src/file_helper.rs` — Helper functions for reading Markdown files and writing the generated HTML file.
- `src/md_to_html_converter.rs` — Core conversion logic that turns Markdown text into HTML.
- `sample.md` — Example Markdown file demonstrating supported syntax.

## Usage

### Build the project

```bash
cargo build
```

### Run the converter

```bash
cargo run -- sample.md
```

This command reads `sample.md`, converts it, and writes a new file named `sample.html` in the same directory.

### Generated output

The tool writes a file using the original file name with the `.html` extension.
For example:

- Input: `notes.md`
- Output: `notes.html`

## Example

The repository includes `sample.md`, which contains headings, paragraphs, a list, a code block, and a quote. Running the project on it creates a ready-to-open HTML page.

## Notes

- The converter supports a basic Markdown subset rather than full Markdown specification compliance.
- Styling is embedded directly in the generated HTML document.
- This project currently does not accept custom CSS or multiple input files via the CLI.

## License

This project is provided as-is for learning and demonstration purposes.
