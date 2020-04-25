# Sockgit

A simple service that allows creation of a new git repository without all of the trouble of having to actually ssh in and remember what to do as your stripped down `git` user.

## Requirements

- `netcat`
- `rustc` stable
- An OpenSSL lib to compile/link against (rewriting in a compiled language was a mistake)

## Compiling on ARM

The whole point of this lib was that it would run on my RPi3+. It doesn't do that off the bat anymore, thanks Rust.

- Compile yourself a version of the OpenSSL source: `curl https://www.openssl.org/source/openssl-$VERSION.tar.gz | tar xz -C /tmp && cd openssl-$VERSION && ./config shared && make`
- Finally build the Piece-of-Software: `OPENSSL_LIB_DIR=/tmp/openssl-$VERSION OPENSSL_INCLUDE_DIR=/tmp/openssl-$VERSION/include cargo build --release`
