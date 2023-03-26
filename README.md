# entity_id

[![Crates.io](https://img.shields.io/crates/v/entity_id.svg)](https://crates.io/crates/entity_id)
[![Docs.rs](https://docs.rs/entity_id/badge.svg)](https://docs.rs/entity_id/)
[![Crates.io](https://img.shields.io/crates/l/entity_id.svg)](https://github.com/maxdeviant/entity-id/blob/master/LICENSE)

Production-grade entity IDs for your web application.

## Usage

```rs
use entity_id::EntityId;
use ulid::Ulid;

#[derive(EntityId, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[entity_id(prefix = "movie")]
struct MovieId(Ulid);

let movie_id = MovieId::new();

println!("{}", movie_id);
// movie_01gwe2pv0c3p1xbcfvm4n8vx08
```
