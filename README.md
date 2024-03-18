# Android Memory Viewer (amv)

Android Memory Viewer (amv) is a command-line tool written in Rust that allows users to view the memory contents of a process running on an Android device. It reads from `/proc/pid/mem`, which requires proper permissions to access.

## Disclaimer

Before using this tool, please ensure that you have the necessary rights to read the memory contents of the process you're targeting.  It might require root access on the device.

## Features

- Read memory contents of Android processes.
- Output memory in both hexadecimal and ASCII formats.
- Simple CLI interface.

## Prerequisites

- A rooted Android device or a device running in debug mode.
- Rust and Cargo installed to build the project.
- Android NDK for compiling Rust for Android if cross-compilation is needed.

## Building

To build the `amv` tool, follow these steps:

```sh
# Clone the repository
git clone https://github.com/dodola/MemoryViewer.git
cd MemoryViewer

# Build the project
cargo build --target aarch64-linux-android --release
```
## Usage
```
./MemViewer 30426 712ec46000 100
0x000000712EC46000 | 64 65 78 0A 30 33 35 00 3B 21 D1 38 2D C6 9A 76  | dex.035.;!.8-..v
0x000000712EC46010 | 80 F2 72 8F 50 F3 9A AF 70 57 FC 43 25 E3 EB 73  | ..r.P...pW.C%..s
0x000000712EC46020 | 44 65 6C 00 70 00 00 00 78 56 34 12 00 00 00 00  | Del.p...xV4.....
0x000000712EC46030 | 00 00 00 00 68 64 6C 00 8E C9 00 00 70 00 00 00  | ....hdl.....p...
0x000000712EC46040 | EC 26 00 00 A8 26 03 00 B6 2C 00 00 58 C2 03 00  | .&...&...,..X...
0x000000712EC46050 | FE 6F 00 00 E0 DA 05 00 41 BC 00 00 D0 5A 09 00  | .o......A....Z..
0x000000712EC46060 | 0E 1D 00 00 
```
