# Hexagonal Chess

## How to Build

### Setup

#### The Rust Toolchain

You will need the standard Rust toolchain, including rustup, rustc, and cargo.

[Follow these instructions to install the Rust toolchain.](https://www.rust-lang.org/tools/install)

#### `wasm-pack`

[Get `wasm-pack` here!](https://rustwasm.github.io/wasm-pack/installer/)

#### `npm`

[Follow these instructions to install npm.](https://www.npmjs.com/get-npm)

If you already have npm installed, make sure it is up to date with this command:

```
npm install npm@latest -g
```

### Build Instructions

```bash
git clone https://github.com/ishaanpathak/hexagonal_chess.git
cd hexagonal_chess
wasm-pack build
```

## How to Run

Note: You need `npm` in order to run the game currently. I am trying a way to make a binary for the game but in the mean time, here's how to run the game:

```bash
cd hexagonal_chess
# Uncomment the following line if you get ssl error.
# export NODE_OPTIONS=--openssl-legacy-provider
npm run start
```

Access the game using this link: http://localhost:8080