# Rust Browser

A simple web browser implementation in Rust, following the principles from [Web Browser Engineering](https://browser.engineering/). This project demonstrates the fundamentals of how web browsers work by building one from scratch.

## Features

- **HTTP/HTTPS Support**: Fetch web pages using HTTP and HTTPS protocols
- **File Protocol**: Load local HTML files
- **Basic HTML Rendering**: Strip HTML tags and display text content
- **Data URLs**: Support for inline data using `data:` scheme
- **View Source**: Support for displaying the raw source of any supported URL using the `view-source:` scheme

### Prerequisites

- Rust (edition 2024)
- Cargo package manager

### Installation

1. Clone the repository:

```bash
git clone <your-repo-url>
cd rust_browser
```

2. Build the project:

```bash
cargo build
```

## Usage

The browser supports multiple ways to load content:

### Run with default test file

```bash
cargo run
```

This creates and loads a test HTML file at `some/directory/example1-simple.html`.

### Load a local file

```bash
cargo run file:///path/goes/here
```

### Load HTTP content

```bash
cargo run http://example.com
```

### Load HTTPS content

```bash
cargo run https://example.com
```

### Load from local server

```bash
cargo run http://localhost:8000/example1-simple.html
```

### Load data URLs

```bash
cargo run "data:text/html,<h1>Hello World</h1>"
```

### View the raw source of a URL

```bash
cargo run "view-source:http://example.com"
```

### Test with HTTPBin

```bash
cargo run https://httpbin.org/get
```

## Development

### Running a local server for testing

You can serve local HTML files using Python's built-in server:

```bash
python -m http.server 8000 -d some\directory
```

Then access files at `http://localhost:8000/example1-simple.html`.

## Dependencies

- [`native-tls`](https://crates.io/crates/native-tls) - TLS/SSL support for HTTPS connections
