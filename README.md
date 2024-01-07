# 🚧🚧🚧 Informatics FRS Helper API 🚧🚧🚧

This project is a RESTful API designed to assist you gather more information related to Informatics FRS Helper. It provides several endpoints to access data related to courses (`/v1/matkul`), lecturers (`/v1/dosen`), and classes (`/v1/kelas`). 

The API is built with scalability and ease of use in mind, and it includes features such as rate limiting to ensure fair usage and prevent abuse. It is also designed to be easily deployable using Docker and Fly.io, making it highly portable and cloud-ready.

In addition to the API itself, the project also includes comprehensive documentation through Swagger and Postman, making it easy for you to understand how to use the various endpoints and what data they can expect to receive.

## Progress 

- [X] `/v1/matkul` endpoint
- [ ] `/v1/dosen` endpoint
- [X] `/v1/kelas` ednpoint
- [ ] Rate limiter
- [ ] Dockerize
- [ ] Deploy to Fly.io
- [ ] Swagger Integration
- [ ] Postman Documentation

## Endpoints

### /v1/matkul
- Dynamic Path : `/v1/matkul/:id_matkul`
- Query params:
    - `nama` (string)
    - `semester` (integer)
    - `sks` (integer)

### /v1/dosen
- Dynamic Path : `/v1/dosen/:id_dosen`
- Query params: 
    - `nama` (string)
    - `kode` (string)

### /v1/kelas
- Dynamic Path : `/v1/kelas/:id_kelas`
- Query params: 
    - `hari` (string)
    - `jam` (string)
    - `matkul` (string)
    - `kode_dosen` (string)

## Tech Stacks

### Rust
Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency. Rust is syntactically similar to C++, but its designers intend it to provide better memory safety while maintaining high performance.

### Axum
Axum is a web application framework that focuses on ergonomics and modularity. It's built on Tokio and Hyper, and it provides everything you need to build robust and efficient web applications with Rust.

### SQLx
SQLx is an async, pure Rust SQL crate featuring compile-time checked queries without a DSL. It enables you to interact with databases directly in Rust, ensuring type safety and connection handling.

### Tokio
Tokio is a Rust framework for developing applications which perform asynchronous I/O — an event-driven approach that can often achieve better performance, lower latency, and greater resource utilization than conventional synchronous I/O.

