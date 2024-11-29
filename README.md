# PDF to JSON Converter

A command-line tool written in Rust that converts PDF documents to structured JSON format, making it easy to process and analyze PDF content programmatically.

## Features

- Extracts text content from PDF files
- Preserves page structure and numbering
- Outputs well-formatted JSON with page-level granularity
- Handles multi-page documents
- Provides clear error messages and robust error handling

## Installation

Ensure you have Rust and Cargo installed on your system. Then clone this repository and build the project:

```bash
git clone [your-repository-url]
cd pdf_to_json
cargo build --release
```

The compiled binary will be available in `target/release/pdf_to_json`.

## Usage

```bash
pdf_to_json <input_pdf_path> <output_json_path>
```

### Arguments:
- `input_pdf_path`: Path to the input PDF file
- `output_json_path`: Path where the output JSON file should be saved

### Example:
```bash
pdf_to_json input.pdf output.json
```

## Output Format

The tool generates a JSON file with the following structure:

```json
{
  "file_name": "example.pdf",
  "total_pages": 2,
  "pages": [
    {
      "page_number": 1,
      "content": "Text content from page 1..."
    },
    {
      "page_number": 2,
      "content": "Text content from page 2..."
    }
  ]
}
```

## Dependencies

- `lopdf`: PDF document handling
- `serde`: Serialization/deserialization framework
- `serde_json`: JSON serialization
- `anyhow`: Error handling
- `chrono`: Date and time functionality

## Requirements

- Rust 2021 edition or later
- Cargo package manager

## Error Handling

The tool includes comprehensive error handling for common scenarios:
- Invalid PDF files
- File access permissions
- Invalid command-line arguments
- Text extraction failures

## License

MIT License
