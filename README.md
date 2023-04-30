myip-rs
=======

A microscopic async Rust http server that reports back the requester's IP address.

This project simply scratched an itch, in some home-brew poor-man's-dynamic-dns tooling
to automate updating a remote bind config when my ISP changes my address.  Yes, there are
plenty of services for this, but why not write one to run *on* the DNS servers to be
updated, which has such a tiny footprint it cannot possibly interfere with a minimal, $3/mo
cloud vm's other tasks.  And was an opportunity to play with minimal async rust server tech,
and a fun evening project minimizing its footprint.

`build.sh` will build a heavily optimized binary (requires Rust's nightly toolchain, as it uses
[`build-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) to compile
and build into the binary only those parts that are needed of Rust's standard library, resulting
in a binary of 326k instead of 1.1Mb.
