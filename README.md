# Cordis

![](https://github.com/HiruNya/cordis/workflows/Rust/badge.svg)

[Documentation](https://hiru.dev/docs/cordis/)

A toy bot framework for Discord. This crate intends on allowing Discord bots to be made utilising the new ``async/await`` syntax.

This crate is split into multiple sub-crates.
- `cordis`: The main framework that you use to make a Discord bot.
- `cordis-core`: Types that are required to communicate with Discord. This crate can be re-used by others who want to make their own Discord frameworks as it has been built to merely map Discord's API. This crate DOESN'T actually carry out the protocol and so does not depend on a HTTP crate, WebSocket crate, or Async runtime.
