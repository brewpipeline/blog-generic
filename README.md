# blog-generic

Shared domain entities, events, and utilities for the blog platform. This crate contains platform-agnostic types used by both the server (`blog-server`) and the frontend (`blog-ui`).

## Modules

### `entities`
Domain model types for the blogging platform:
- **Author** / **AuthorContainer** / **AuthorsContainer** - Author profiles and collections
- **Post** / **PostContainer** / **PostsContainer** - Blog posts and collections
- **Comment** / **CommentsContainer** - Post comments
- **Tag** / **TagContainer** - Post tags
- **Login** - Authentication request types
- **Chat** - Chat message types
- **CommonPost** / **CommonComment** / **CommonMinimalAuthor** / **CommonSecondaryAuthor** - Shared trait abstractions
- **TotalOffsetLimitContainer** - Paginated response wrapper

### `events`
Async event definitions for the event bus:
- **NewPostPublished** - Fired when a new post is published
- **SubscriptionStateChanged** - Fired when a subscription state changes

### `author_slug_utils`
Utilities for manipulating author slugs with a `^` separator (clean/extend operations).

### `page_processor`
Pagination logic with `PageProcessor` trait and `DefaultPageProcessor<LIMIT>` implementation for offset-based pagination.

## Features

- `validator` - Enables [validator](https://crates.io/crates/validator) derive macros for request validation

## Dependencies

- `serde` - Serialization/deserialization with derive support
- `validator` (optional) - Struct validation

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
blog-generic = { path = "../blog-generic" }
```

With validation support:

```toml
[dependencies]
blog-generic = { path = "../blog-generic", features = ["validator"] }
```
