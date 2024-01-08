# 🚧🚧🚧 Informatics FRS Helper API 🚧🚧🚧

This project is a RESTful API designed to assist you gather more information related to Informatics FRS Helper. It provides several endpoints to access data related to courses (`/v1/matkul`), lecturers (`/v1/dosen`), and classes (`/v1/kelas`).

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

## Documentation

For more detailed information about the API and its usage, please refer to this [Postman Documentation](https://documenter.getpostman.com/view/30505077/2s9YsJBCJo). Postman is a great tool for interacting with our API endpoints. To get started, simply click the button below to import the pre-configured collection into your Postman application.

[![Run in Postman](https://run.pstmn.io/button.svg)](https://app.getpostman.com/run-collection/30505077-3870bf0a-a82f-486e-9385-22ad91100a56?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D30505077-3870bf0a-a82f-486e-9385-22ad91100a56%26entityType%3Dcollection%26workspaceId%3D307a9cd1-e40a-4022-ae90-d057479f2a88#?env%5Bprod%5D=W3sia2V5IjoiQkFTRV9VUkwiLCJ2YWx1ZSI6Imh0dHBzOi8vYXBpLWluZm9ybWF0aWNzLWZycy1oZWxwZXIuZmx5LmRldiIsImVuYWJsZWQiOnRydWUsInR5cGUiOiJkZWZhdWx0Iiwic2Vzc2lvblZhbHVlIjoiaHR0cHM6Ly9hcGktaW5mb3JtYXRpY3MtZnJzLWhlbHBlci5mbHkuZGV2Iiwic2Vzc2lvbkluZGV4IjowfSx7ImtleSI6IkFQSV9WRVJTSU9OIiwidmFsdWUiOiJ2MSIsImVuYWJsZWQiOnRydWUsInR5cGUiOiJkZWZhdWx0Iiwic2Vzc2lvblZhbHVlIjoidjEiLCJzZXNzaW9uSW5kZXgiOjF9XQ==)

## Tech Stacks

### Rust
Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency. Rust is syntactically similar to C++, but its designers intend it to provide better memory safety while maintaining high performance.

### Axum
Axum is a web application framework that focuses on ergonomics and modularity. It's built on Tokio and Hyper, and it provides everything you need to build robust and efficient web applications with Rust.

### SQLx
SQLx is an async, pure Rust SQL crate featuring compile-time checked queries without a DSL. It enables you to interact with databases directly in Rust, ensuring type safety and connection handling.

### Tokio
Tokio is a Rust framework for developing applications which perform asynchronous I/O — an event-driven approach that can often achieve better performance, lower latency, and greater resource utilization than conventional synchronous I/O.

