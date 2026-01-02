# Radula

A Rust library for scraping the [Divinuum Officium](https://www.divinumofficium.com/) website.

## Getting started

This project is in an early development state. The library API is not stable and is subject
to change.

### Example application

The `docli` application is provided as an example usage of the library. It provides a simple command
line interface to fetch and display the vespers for the specific day.

The application can be build using the following command:

```shell
cargo build --example docli
```

Usage instructions are available under the command `docli --help`.

### Local copy of Divinuum Officium

Example application and tests require a local instance of Divinuum Officium served at
`http://127.0.0.1:8080`.

The local instance can be easily set up using Podman containers:

```shell
git clone https://github.com/DivinumOfficium/divinum-officium.git
cd divinum-officium
podman-compose up
```
