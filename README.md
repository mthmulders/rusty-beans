# Rusty Beans

A pet project to experiment with reading Java class files in Rust

[![Build status](https://github.com/mthmulders/rusty-beans/actions/workflows/build.yml/badge.svg)](https://github.com/mthmulders/traqqr/actions/workflows/build.yml)

Rusty Beans is a personal pet project.
It serves two main goals

1. The primary goal is to explore the [Java class file](https://en.wikipedia.org/wiki/Java_class_file) format.
2. The secondary goal is to deepen my understanding of the [Rust](https://www.rust-lang.org/) programming language.

## Technologies

The project uses the following technologies:
- [Rust](https://www.rust-lang.org/)

## Project Structure

This project is split in a few subprojects:

* [cafebabe](tree/main/cafebabe) contains a [library crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) to read class files.
* [rjvm](tree/main/rjvm) contains another [library crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) to execute one or more class files.
* [java](tree/main/java) contains a [binary crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) that mimics the `java` executable which comes with the Java Virtual Machine.

## Documentation

There is no documentation yet.

## License

The code is licensed under the MIT license.
See [./LICENSE](LICENSE) for details.

Java is a registered trademarks of Oracle and/or its affiliates.
Other names may be trademarks of their respective owners.
