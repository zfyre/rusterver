# Simple Rust Server

This repository contains a minimal Rust server with:
- A thread pool for parallel request handling
- Graceful shutdown logic for safe resource cleanup

## Building and Running
1. Install Rust (https://www.rust-lang.org/tools/install).
2. Clone this repo and run:
    ```bash
    cargo run
    ```

## Features
- Thread pooling to handle incoming connections concurrently
- Clean shutdown initiated via a signal, ensuring all active tasks complete

Feel free to explore and modify to suit your requirements!