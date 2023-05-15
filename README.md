## Contact

Rik Arends: https://fosstodon.org/@rikarends#

Eddy Bruel: - 

Sebastian Michailidis: -

Our discord channel for Makepad:
https://discord.gg/adqBRq7Ece

# Makepad

## Overview
 
This is the repository for Makepad, a new way to build UIs in Rust for both native and the web.

Makepad consists of Makepad Framework and Makepad Studio.

Makepad Framework is our UI framework. It consists of multiple crates, but the top level crate is [makepad-widgets](https://crates.io/crates/makepad-widgets). For a further explanation of Makepad Framework, please see the README for that crate.

Makepad Studio is a prototype of an IDE that we've built using Makepad Framework. It's still under heavy development, but our eventual goal with Makepad Studio is to create an IDE that enables the design of an application to be changed at runtime. The main crate for Makepad Studio is [makepad-studio](https://crates.io/crates/makepad-studio). Please see the README for that crate for more.

Demo links:

[makepad-example-fractal-zoom](https://makepad.nl/makepad/examples/fractal_zoom/src/index.html)

[makepad-example-ironfish](https://makepad.nl/makepad/examples/ironfish/src/index.html)

[makepad-example-simple](https://makepad.nl/makepad/examples/simple/src/index.html)

[makepad-example-numbers](https://makepad.nl/makepad/examples/numbers/src/index.html)

### Prerequisites

NOTE: At the moment, we only support Mac and web. We however have most of the code for Windows and Linux already there and it will be supported in the near future.

To build the Makepad crates you first need to install Rust.\
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Our native builds work on the stable Rust toolchain. However, some of the errors generated by Makepad at runtime (particulary those originating in our DSL) do not contain line information unless you use the nightly Rust toolchain. Moreover, our web builds only work on nightly for now. For this reason, we recommend that you build Makepad using the nightly Rust toolchain.

In order to install the nightly Rust toolchain, run:
```rustup toolchain install nightly```

For our web builds you need to install the wasm32 compilation target:\
```rustup target add wasm32-unknown-unknown --toolchain nightly```\

And then add the rust std library source for compiling the threads and wasm features:\
```rustup component add rust-src --toolchain nightly```

## Build Instructions

The build instructions you see here are for Ironfish, a feature rich synthesizer, and the first real example application for Makepad Framework. Ironfish is also available as a standalone crate at: [https://crates.io/crates/makepad-example-ironfish].

Make sure you have all the prerequisites above installed first!

### Native

To build and run the native version of Ironfish, run the following command from the root directory of the repository:
```cargo +nightly run -p makepad-example-ironfish --release -F nightly```

### Web

To build the web version of Ironfish, run the following command from the root directory of the repository:
```tools/build_wasm_thread.sh makepad-example-ironfish```

After the build is complete, run the following command to start our web server:
```cargo run -p makepad-web-server --release```

After starting the web server, the web build of Ironfish should be available here:
https://127.0.0.1:8080/makepad/examples/ironfish/src/index.html


