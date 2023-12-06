# color-picker-slint

<div align="center"><sub> ⚠️ Work in progress ⚠️</sub></div>

This color picker is a cross-platform desktop application developed using Rust and the Slint UI framework. It allows users to easily pick colors from anywhere on their screen. Currently, the application is in active development, with the screen color picking feature nearing completion.

## Features

- [x] **Cross-Platform Compatibility**: Runs on Windows, macOS, and Linux.
- [x] **Real-Time Color Picking**: Users can pick colors in real-time from any location on their screen.
- [x] **Magnifier Tool**: A magnifier tool to zoom in on pixels for precise color selection.
- [x] **Color Format Conversion**: Supports conversion and display of colors in multiple formats like RGB, HEX, etc.
- [ ] **Intuitive User Interface**: Built with Slint UI, offering a user-friendly and straightforward interface.
- [ ] **Lightweight and Efficient**: Leveraging Rust's performance for a fast and low-resource application.

## Installation

The application is currently in development. A downloadable version for various operating systems will be available upon completion.

For those interested in building from source, follow these steps:

```bash
git clone https://github.com/luckychacha/color-picker-slint.git
cd color-picker-slint
cargo build --release
```

After building, the executable will be located in the `target/release` directory.

## Usage

Once launched, you can click on any location on your screen to pick a color. The picked color will be displayed in the application interface, where you can view and copy it in different formats. (More features coming soon)

## Contributing

Contributions of all kinds are welcome, including feature requests, feedback, or code contributions. Please read the `CONTRIBUTING.md` file to understand how to get started.

## License

This project is licensed under the [MIT License](LICENSE).
