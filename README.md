# Demo SurrealDB from embedded ESP32 (with std)

This is a demo project that queries SurrealDB from a ESP32C3 RISCV controller.

Continuous integration:
CI:
[![CircleCI](https://circleci.com/gh/flyaruu/surreal-rust-esp.svg?style=svg)](https://circleci.com/gh/flyaruu/surreal-rust-esp)

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

## Running this demo

Note from one backend developer to another: Developing for embedded targets can be a bit rough. Things tend to have more sharp edges and things tend to fail in unhelpful ways. You can figure this out - none of it is magic - but do expect to need some time to get it all working.

Also we're working with hardware. Hardware can have intermittent issues. Hardware can be sensitive to environments: Maybe it works when it's cold, but not when it is warm. Maybe a connection is wonky. Maybe a cable is bad.

My point: When developing software, we tend to assume that the hardware 'just works', but when developing embedded, we don't have that luxury any more.

### Get some hardware

- To run this demo, you'll need an esp32c3 board. They are easy to get and shouldn't cost more than €10. Look for one on Amazon or someplace else. You can also use esp32s2 / eps32s3 boards, it should be possible to get these to work, but note that they do have another architecture (xtensa instead of RiscV) so some changes will be made.
- You need a micro USB cable. It needs to be a _data_ cable, not just a power cable. If you use a cable that was supplied just to charge some device (like headphones) that cable may not work.
 
- First, clone (or fork) this repo.
- Make sure Rust is installed, I suggest you start here: https://www.rust-lang.org/tools/install
- We need to run an instance of surrealdb somewhere. I tend to run locally, so you can run 'docker-compose up' to start the example database locally.
- Then we'll need to set tell the device to a) set up wifi b) tell it how to reach the surrealdb instance. We define those using environment variables:
```
export RUST_ESP32_STD_DEMO_WIFI_SSID=<my ap>
export RUST_ESP32_STD_DEMO_WIFI_PASS=<my pass>
export SURREALDB_ENDPOINT=http://<surrealdb ip>:8000
```
If you are running surrealdb locally, find your ip on the local network (use ifconfig or something). You can not use your public ip, it must be the ip the esp32 device can reach your computer on.

In my case it is 10.11.12.177
I can check locally: 10.11.12.177:8000/health it shouldn't return anything, but also no error.

Note that the device itself has no notion of environment variables, as they are an operating system feature. These will be expanded and hard coded in the binary while compiling, so you will need to do this _before_ compiling, and if you change them, you'll need to recompile.

- Take a look at the .circleci/config.yml file. This continuous integration builds on every commit (hopefully the badge above is green). Note that this CI _only builds_, there are no actual runs or tests, but in this case the building is harder than the running, as the demo doesn't do much.
- The circle ci environment runs in Ubuntu linux, so if you are running that, the commands should match exactly. Otherwise there will be slight differences, mostly around installing the python environment (Used for building for ESP-IDF)
- Open the project (I use vscode, other editors might need tweaking). Make sure rust-analyzer has been installed.
- Check the .vscode/settings.json file, it describes the platform used + some required compiler settings.
- When running cargo from the command line, it should follow the target + compiler settings defined in .cargo/cargo.toml, so no extra settings should be needed.

Let's try:
```
cargo build --release
```
Takes quite a while, as the standard library and FreeRTOS integration isn't supplied in the target, it is recompiled with our code, so it is actually quite some code (subsequent compilations will be quicker). But hopefully it ends like this:

```
   Compiling futures v0.3.28
   Compiling simplehttp v0.0.1 (https://github.com/flyaruu/simplehttp#bdb888ea)
   Compiling surrealdb-http v0.1.0 (https://github.com/flyaruu/surrealdb-http#e54b835b)
    Finished release [optimized] target(s) in 3m 46s
➜  surreal-rust-esp git:(main) ✗ 
```
- Now connect your device to a USB port and run:
  ```cargo espflash --monitor```

  It should flash the board (=upload the binary) and restart the board. It will try to connect to Wifi, and start listing fictional actors:

  ```
  I (9848) HTTP_CLIENT: Body received in fetch header state, 0x3fca8462, 166
[{"time":"1.38122ms","status":"OK","result":[{"actor_id":1,"first_name":"Penelope","id":"actor:1","last_name":"Guiness","last_update":"2013-05-26T14:47:57.620000"}]}]
Actor: Actor { first_name: "Penelope", last_name: "Guiness", id: "actor:1" }
I (11048) HTTP_CLIENT: Body received in fetch header state, 0x3fca8462, 164
[{"time":"1.523129ms","status":"OK","result":[{"actor_id":2,"first_name":"Nick","id":"actor:2","last_name":"Wahlberg","last_update":"2013-05-26T14:47:57.620000"}]}]
Actor: Actor { first_name: "Nick", last_name: "Wahlberg", id: "actor:2" }
I (12088) HTTP_CLIENT: Body received in fetch header state, 0x3fca8462, 159
[{"time":"1.220574ms","status":"OK","result":[{"actor_id":3,"first_name":"Ed","id":"actor:3","last_name":"Chase","last_update":"2013-05-26T14:47:57.620000"}]}]
Actor: Actor { first_name: "Ed", last_name: "Chase", id: "actor:3" }
```
Underwhelming? Perhaps. I'll try to make a more compelling demo at some point, but the difficult parts are working.

And performance isn't too terrible: If I remove the sleep statement, our dinky little microcontroller does about 100 requests per second to the database. It's not an amazing number, but more than enough for many use cases.

