# RustAutoBacklight(rab): Adaptive Screen Brightness with Rust

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust) ![Linux](https://img.shields.io/badge/OS-Linux-blue?logo=linux)

A Rust-based Linux utility that automatically adjusts laptop screen backlight based on ambient light conditions captured through the webcam.

## Features

- 📷 Real-time ambient light analysis using webcam
- 🌓 Smooth backlight adjustment based on light levels
- ⚙️ Configurable brightness thresholds and adjustment ranges
- 🖥️ Supports most Linux laptops with webcam and backlight control
- 🔒 Privacy-focused (no image storage/transmission)

## Installation

### Prerequisites

- Rust 1.70+
- Linux OS with kernel 5.10+
- v4l-utils (for webcam access)
- Build essentials

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/mamad-zahiri/rab.git
   cd autobacklight
   ```

2. Install dependencies

   - (Debian/Ubuntu):

     ```bash
     sudo apt install rustupn build-essential libv4l-dev pkg-config mplayer
     ```

   - (Arch):

     ```bash
     sudo apt install rustup mplayer
     ```

3. Build and install:

   ```bash
   cargo build --release
   sudo cp target/release/rab /usr/local/bin/
   ```

## Usage

Basic usage:

```bash
sudo rab
```

## How It Works

1. **Image Capture**: Captures low-resolution grayscale frames from webcam
2. **Grayscaling**: Grayscale the taked image
3. **Calculate Brightness**: Sum of gray pixels devided to image size devided to 255 multipy by (max system brightness - min system brightness)
4. **Backlight Adjustment**: Maps brightness values to configured backlight range
5. **Hardware Interface**: Writes to sysfs backlight control interface

## Dependencies

- [confy](https://docs.rs/confy/latest/confy/) - Zero-boilerplate configuration management
- [image](https://docs.rs/image/latest/image/) - A Simple-to-use, cross-platform Rust Webcam Capture Library
- [serde](https://serde.rs/) - Serializing and Deserializing Rust data structures
- [serde_drive](https://serde.rs/derive.html) - macro to generate implementations of the Serialize and Deserialize traits

## Limitations

- Requires compatible webcam and backlight interface
- Webcam must not be in use by other applications
- May need permissions adjustment for video devices

## Security Note

This application:

- Processes images in memory only
- Never stores or transmits captured frames
- Requires explicit user permission for camera access

## Contributing

PRs and issues welcome! Please follow:

1. Fork the repository
2. Create your feature branch
3. Commit changes with descriptive messages
4. Push to the branch
5. Open a PR
