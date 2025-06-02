# sql-grimoire-id

A Rust library for creating strongly-typed, prefixed UUID identifiers with built-in serialization support.

## Features

- **Type-safe IDs**: Create distinct identifier types to prevent mixing different kinds of IDs
- **Prefixed serialization**: IDs serialize to strings with configurable prefixes (e.g., `user-550e8400-e29b-41d4-a716-446655440000`)
- **Serde integration**: Automatic serialization/deserialization support
- **UUID-based**: Built on the robust `uuid` crate

## Usage

Use the `define_id!` macro to create new ID types:

```rust
use sql_grimoire_id::define_id;

define_id!(UserId, "user");
define_id!(PostId, "post");

// Create IDs from UUIDs
let user_id = UserId::from(uuid::Uuid::new_v4());
let post_id = PostId::from(uuid::Uuid::new_v4());

// IDs serialize with their prefix
println!("{}", user_id); // "user-550e8400-e29b-41d4-a716-446655440000"
println!("{}", post_id); // "post-6ba7b810-9dad-11d1-80b4-00c04fd430c8"
```

## Serialization

IDs automatically serialize to prefixed strings and deserialize from them:

```rust
use serde_json;

let user_id = UserId::from(uuid::Uuid::new_v4());
let json = serde_json::to_string(&user_id).unwrap();
// json = "\"user-550e8400-e29b-41d4-a716-446655440000\""

let deserialized: UserId = serde_json::from_str(&json).unwrap();
assert_eq!(user_id, deserialized);
```

## Time-Ordered IDs

For applications that need chronologically ordered identifiers, use UUID v7:

```rust
// Create time-ordered IDs that sort chronologically
let user_id = UserId::from(uuid::Uuid::now_v7());
let post_id = PostId::from(uuid::Uuid::now_v7());

// IDs created later will have lexicographically larger string representations
// This is useful for database indexing and sorting
```

UUID v7 embeds a timestamp, making IDs naturally ordered by creation time. This is particularly valuable for:
- Database primary keys that benefit from sequential insertion
- Reducing index fragmentation in time-series data
- Maintaining chronological order without separate timestamp fields

## Dependencies

- `serde` - Serialization framework
- `uuid` - UUID generation and parsing