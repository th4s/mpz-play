<p align="center">
    <img src="./mpc-ferris.png" width=1280 />
</p>

# MPZ Play

This is a workshop to get familiar with [mpz](https://github.com/privacy-scaling-explorations/mpz),
which is a [PSE](https://pse.dev) library for [secure multi-party computation](https://en.wikipedia.org/wiki/Secure_multi-party_computation) (MPC).

## Resources
Learning MPC:
- [Secret Sharing MPC](https://eprint.iacr.org/2022/062)
- [SecureComputation](https://securecomputation.org/)
- [BIU MPC Lectures](https://www.youtube.com/playlist?list=PL8Vt-7cSFnw1F7bBFws2kWA-7JVFkqKTy)
- [CO15 - Simple Oblivious Transfer](https://eprint.iacr.org/2015/267)
- [MASCOT](https://eprint.iacr.org/2016/505)

MPZ library:
- [MPZ Overview](https://github.com/privacy-scaling-explorations/mpz/blob/dev/README.md)
- [MPZ Design Considerations](https://github.com/privacy-scaling-explorations/mpz/blob/dev/DESIGN.md)

## Getting started

This repository offers some exercises to get exposed to MPC with mpz. Each of
these exercises are Rust crates, that start with a number, e.g. `00-connect`. In
each of theses crates you will find two binaries `alice.rs`, `bob.rs` and a
library file `lib.rs`. This library file contains instructions and typically you
will need to add some code to the binaries. To complete the exercises run the
binaries and see if your code works as expected.

For example to complete lesson `00-connect`, you follow the instructions in
`00-connect/src/lib.rs`. Then in two different terminals run

```sh
cargo run -p connect --bin alice
cargo run -p connect --bin bob
```

Since we use a TCP connection for connecting `Alice` and `Bob`, you can work
through the exercises together with a colleague. Then each of you only needs to
run one binary, either `alice.rs` or `bob.rs`, and you can connect to your
colleague's machine. This way you can do pair programming and real MPC on
different machines :)

