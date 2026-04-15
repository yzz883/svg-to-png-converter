# SVG to PNG Converter

A modern web-based tool for converting SVG files to PNG images, built with Rust backend and a beautiful frontend interface.

## Features

- 🎨 **Beautiful UI**: Modern, dark-themed interface with smooth animations
- 📤 **Multiple Input Methods**: Upload SVG files or paste SVG code directly
- 📏 **Custom Dimensions**: Set custom width and height for output PNG
- ⚡ **Fast Conversion**: Client-side conversion using Canvas API
- 💾 **Easy Download**: One-click download of converted PNG images

## Tech Stack

- **Backend**: Rust + Actix-web
- **Frontend**: Pure HTML/CSS/JavaScript (no frameworks)
- **Conversion**: HTML5 Canvas API

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yzz883/svg-to-png-converter.git
cd svg-to-png-converter
```

2. Run the server:
```bash
cargo run
```

3. Open your browser and navigate to:
```
http://127.0.0.1:8081
```

## Usage

1. **Upload SVG File**: Drag and drop an SVG file onto the upload area, or click to browse
2. **Or Paste Code**: Switch to the "Paste Code" tab and paste your SVG code
3. **Set Dimensions**: Adjust the output width and height as needed
4. **Convert**: Click the "Convert to PNG" button
5. **Download**: Click "Download PNG" to save the converted image

## Project Structure

```
svg-to-png-converter/
├── Cargo.toml          # Rust project configuration
├── src/
│   └── main.rs         # Backend server code
├── static/
│   └── index.html      # Frontend page
├── 开发计划.md          # Development plan (Chinese)
└── 开发日志.md          # Development log (Chinese)
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
