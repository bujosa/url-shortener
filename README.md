# URL Shortener
This is a straightforward project to create your own URL shortening service using Redis and Rust.

## Prerequisites

- [Redis](https://redis.io/) - Used for caching.
- [Rust](https://www.rust-lang.org/) - The project is built using Rust.

## Running the Project

1. Ensure you have the prerequisites installed.
2. Clone the repository.
3. Navigate to the project directory.
4. Run the project using Cargo:

```bash
cargo run
```

## How to use

### Create a short URL

You can create a short URL by making a POST request to the `/api/shorten` endpoint. Here's an example using `curl`:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"url": "https://www.google.com"}' http://localhost:8080/api/shorten
```

### Use a short URL

You can use a short URL by making a GET request to the `/api/<key>` endpoint. Here's an example using curl:

```bash
curl -v http://localhost:8080/api/<key>
```

## References

- [Actix web](https://actix.rs/) - The web framework used.
- [Redis](https://redis.io/) - The in-memory data structure store used.
- [Redis-rs](https://github.com/redis-rs/redis-rs)  - The Redis library for Rust used.
- [Rust](https://www.rust-lang.org/) - The language the project is built in.