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

# Rust HTTP Server Documentation

## Overview
A basic HTTP server implementation in Rust that handles GET requests with support for:
- Basic routing
- Response handling
- Sleep endpoints for testing
- Content serving

## Core Components

### Connection Handler
`handle_connection(mut stream: TcpStream)`
- Primary function for processing incoming TCP connections
- Handles request parsing and response generation
- Buffer size: 1024 bytes

### Request Types
Supports the following endpoints:
1. `GET /` 
    - Standard route with 1 second delay
    - Returns 200 OK with index.html
2. `GET /sleep`
    - Testing endpoint with 5 second delay
    - Returns 200 OK with index.html
3. Other routes
    - Returns 404 NOT FOUND with 404.html

### Response Format
Follows HTTP/1.1 standard with structure: