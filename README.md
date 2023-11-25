# Circle Payment Service

[![Crates.io](https://img.shields.io/crates/v/circle_api?style=flat-square)](https://crates.io/crates/circle_api)

A heavy WIP implementation of Circle's payment API in Rust. Mostly following this guide: https://learn.circle.com/quickstarts/dev-controlled-wallets

## Usage

In `.env`
```
CIRCLE_API_KEY=your_api_key
CIRCLE_ENTITY_SECRET=your_entity_secret
```

And then:
```
cargo run
```