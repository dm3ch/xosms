# Xosms [![Build & Test](https://github.com/NovusTheory/xosms/actions/workflows/build-test.yml/badge.svg?branch=dev)](https://github.com/NovusTheory/xosms/actions/workflows/build-test.yml) [![npm version](https://badge.fury.io/js/xosms.svg)](https://badge.fury.io/js/xosms)
A cross platform media service library made in Rust for Node to easily and seamelessly integrate with the operating systems media service API.

## Current Platforms Supported
- [x] Windows
- [ ] MacOS
- [x] Linux (via MPRIS)

Even if your platform above isn't currently supported, the beauty of xosms is that it will still compile for it but noop on everything.

# Development
To setup and locally develop and build xosms please ensure you have
- Rust
- Yarn

Once you have all of the above you can clone the repository and run
- `yarn`
- `yarn build` (Compiles the rust natives)
- `yarn build:lib` (Runs tsc)
