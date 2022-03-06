## Intro
Counter app build with yew. My first training to build web app using yew.

### Prerequesites
- Rust
- Node

### Getting Started

#### Install WebAssembly Target
```bash
$ rustup target add wasm32-unknown-unknown
```

#### Install Trunk
```bash
$ cargo install trunk
```

#### Clone this repo
```bash
$ git clone https://github.com/nukumalik/yew-counter.git
$ cd yew-counter
```

#### Install Node Packages
using NPM:
```bash
$ npm install
```
using yarn:
```bash
$ yarn install
```

#### Run App
```bash
$ trunk serve
```
Open the app on localhost:8080

### Third-Party Packages
- yew
- bootstrap