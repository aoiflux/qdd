# QDD - Quick Data Duplicator

**QDD (Quick Data Duplicator)** is a high-performance file copier that leverages **zero-copy** techniques to efficiently copy files and create disk images. Designed for speed and efficiency, QDD minimizes unnecessary data movement, making it an ideal tool for large-scale file transfers and disk cloning.

## Features

- **Zero-Copy File Duplication**: Uses OS-level optimizations to reduce CPU and memory overhead.
- **Fast Disk Imaging**: Can be used to create and restore disk images with minimal performance impact.
- **Minimal Resource Usage**: Avoids unnecessary buffer allocations and data copies.

## Installation
To install QDD, ensure you have Rust installed and then build from source:

```bash
git clone https://github.com/aoiflux/qdd.git
cd qdd
cargo build --release
