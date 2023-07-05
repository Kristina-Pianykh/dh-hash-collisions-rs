# Hash collisions in Deffie-Hellman Key Exchange

This program is based on the concept of the so-called [Birthday attack](https://en.wikipedia.org/wiki/Birthday_attack#:~:text=A%20birthday%20attack%20is%20a,between%20two%20or%20more%20parties) to find hash collisions in the context of the [Deffie-Hellman Key Exchange](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange).

## Context

Alice and Bob generate their own private and public keys. They then exchange their public keys and compute the shared secret using their own private key and the public key of the other. The result is hashed using [SHA3-224](https://en.wikipedia.org/wiki/SHA-3) and verified over the phone (it is supposed to be identical).

 ## Attack implementation
 
The idea is to intercept the public keys of both Alice and Bob and establish a man-in-the-middle, pretending to be Alice for Bob and Bob for Alice. The program brute-forces private keys for each so that, when hashed, the computed shared secret in "Attacker-Alice" and "Attacker-Bob" communication would have the same first `n` hex digits (the assumption is that Alice and Bob would stop verifying the hashed secret over the phone after `n` digits).

## Execute

The program takes three args: the public key of Alice in hex, the public key of Bob in hex and the number of the first hex digits of the shared hashed secret to collide

```
docker build -t collisions .
docker run collisions <Alice_pub_key> <Bob_pub_key> <hash_collision_prefix>
```

where `hash_collision_prefix` is the first `n` hex digits of the matching hashed shared secret.

Note: I haven't tried setting `hash_collision_prefix > 12`.

## Runtime

The main motivation behind this program is to reduce long execution times caused by heavy cryptographic computation. It was written to compare different Rust libraries for big numbers and compared to an identical program written in Python (Standard library, no optimizations).
The following table provides results for the 12 first hex digits of the hashed shared secret. For Rust programs, the optimized version was used (`cargo run --release`).

| Language  | Library | Num type | Appr. Runtime | Commit |
| ------------- | ------------- | ----- | ----- | --- |
| Python  | Standard Library  | int | ~1h:15m | |
| Rust  | [num_bigint](https://docs.rs/num-bigint/latest/num_bigint/) | BigInt  | ~1h:40m | [e50b4ec](https://github.com/Kristina-Pianykh/dh-hash-collisions-rs/commit/e50b4ec2f79d0623dfe7d14344d98d95e0216cfd) |
| Rust | [openssl](https://docs.rs/openssl/latest/openssl/bn/struct.BigNum.html) | BigNum | ~16m | [0c92339](https://github.com/Kristina-Pianykh/dh-hash-collisions-rs/commit/0c923397a2766d9e89bdf816dbb84ac5c9cb19eb) |
| Rust | [rug](https://docs.rs/rug/1.19.2/rug/) | Integer | ~16m | latest version on `main` |   
