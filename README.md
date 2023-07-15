# Demo SurrealDB from embedded ESP32 (with std)

This is a demo project that queries SurrealDB from a ESP32C3 RISCV controller.

## Why?
An investigation on running a 'normal' microservice on a microcontroller. Many microservices we make don't need a lot of compute power, and we also don't need a 'full' operating system in many cases.

The esp32c3 specs are pretty sad compared to a 'real' computer:
 - 400kb of RAM
 - 240Mhz processor

But you can do a _lot_ with this if you use a low level language and don't have things you don't need.

Expressif (the company behind ESP32) offers the ESP-IDF platform, a C based runtime that offers a 'operating system lite' (FreeRTOS compatible) that makes it a bit easier to do system calls.

It also allows the Rust std library to be used, which greatly improves compatibility of 3rd party crates.

## Why is it hard?
Many crates, and especially when doing 'microservicy' stuff, assume a normal computer with a normal OS, which we don't have.

Much of the Rust ecosystem uses Tokio, which isn't supported at the moment. It is not easy to make that happen (See this discussion: https://github.com/ivmarkov/rust-esp32-std-demo/issues/153)

So in order to make this work we need to sidestep these crates. I do this by using an ESP-IDF service 'EspHttpConnection' to get an http client, and providing that to the surrealdb client

Big thanks to ivmarkov, his work on ESP-IDF with Rust is invaluable, this demo is loosely based on this demo:
https://github.com/ivmarkov/rust-esp32-std-demo

