# NAT Punchthrough
Simple Rust implementation of [NAT hole punching](https://en.wikipedia.org/wiki/Hole_punching_(networking)) technique for P2P needs, with no dependencies.

![image](https://github.com/sfurkan20/NAT-Punchthrough/assets/82230659/0402939d-d40b-4ce4-9245-0def6aea2d55)

## Build
`cargo build --release`

## Server
Running the server requires no additional command line arguments.

## Client
To run the client:

`client <user group identifier string> <inbounding port>`

The host data of a client will only be shared with other peers with the same group identifier.
