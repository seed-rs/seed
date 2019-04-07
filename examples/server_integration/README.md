# Server integration

A demonstration of sharing data structures between client and server when
using a Rust backend to avoid duplication and incompatibilities. It uses
serde for data de/serialization and Rocket for the backend. It also contains
a simple exmaple for a get request.

## Execute

First run either `build.sh` or `build.ps1` depending on your operating
system to compile and package the client, and afterwards start the
server by executing `cargo +nightly run`.
