# cosmic-login1-zbus

D-Bus bindings for the interfaces provided by `systemd-logind`
(`org.freedesktop.login1`), built on [zbus](https://crates.io/crates/zbus).

The crate provides a convenient Rust API abstraction of the logind D-Bus
interface, parsing responses into concrete structs and enums where possible.

Reference: <https://www.freedesktop.org/software/systemd/man/org.freedesktop.login1.html>

This crate is a fork of [`logind-zbus`](https://gitlab.com/flukejones/logind-zbus) by Luke D Jones.
