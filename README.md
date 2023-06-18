# Snapcraft

[![Crates.io](https://img.shields.io/crates/l/snapcraft)](https://github.com/a1ecbr0wn/snapcraft-wmi/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/snapcraft)](https://crates.io/crates/snapcraft) [![Build Status](https://github.com/a1ecbr0wn/snapcraft/actions/workflows/build.yml/badge.svg)](https://github.com/a1ecbr0wn/snapcraft/actions/workflows/build.yml) [![docs.rs](https://img.shields.io/docsrs/snapcraft)](https://docs.rs/snapcraft) [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/snapcraft/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/snapcraft) [![snapcraft.io](https://snapcraft.io/rust-example/badge.svg)](https://snapcraft.io/rust-example)

A Rust library giving access to the snapcraft standard environment variables for when you need to alter your behaviour when your application is running inside a snap, e.g. you need to find the user's home folder to access some shared settings, or you need to work out where the data folder of the snap is so that you can write to it.

This repository also contains an example application that will be pushed to the snapcraft store as `rust-example` by the automated build.

[![Get the example app from the Snap Store](https://snapcraft.io/static/images/badges/en/snap-store-black.svg)](https://snapcraft.io/rust-example)
