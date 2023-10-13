# NAT Punchthrough
Simple Rust implementation of [NAT hole punching](https://en.wikipedia.org/wiki/Hole_punching_(networking)) technique for P2P needs, with no dependencies.

![image](https://github.com/sfurkan20/NAT-Punchthrough/assets/82230659/ffda51d6-bc2d-4930-8bf4-b403c81e9195)

## Build
`cargo build --release`

## Server
Running the server requires no additional command line arguments.

## Client
To run the client:

`client <user group identifier string> <inbounding port>`

The host data of a client will only be shared with other peers with the same group identifier.
