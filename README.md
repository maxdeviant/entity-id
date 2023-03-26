# entity_id

[![crates.io](https://img.shields.io/crates/v/entity_id.svg)](https://crates.io/crates/entity_id)
[![docs.rs](https://docs.rs/entity_id/badge.svg)](https://docs.rs/entity_id/)
[![crates.io](https://img.shields.io/crates/l/entity_id.svg)](https://github.com/maxdeviant/entity-id/blob/master/LICENSE)
[![CI](https://github.com/maxdeviant/entity-id/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/maxdeviant/entity-id/actions/workflows/ci.yml)

Production-grade entity IDs for your web application.

## Features

- All the features of [ULIDs](https://github.com/ulid/spec)
- Prefixed IDs à la Stripe (`cus_01gwfyayqspvsdqzd32nh44psh`)
- Easy conversion to and from UUIDs

## Usage

```rust
use entity_id::EntityId;
use ulid::Ulid;

#[derive(EntityId, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[entity_id(prefix = "movie")]
struct MovieId(Ulid);

let movie_id = MovieId::new();

println!("{}", movie_id);
// movie_01gwe2pv0c3p1xbcfvm4n8vx08
```
