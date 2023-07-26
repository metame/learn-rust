## Backend Web Development with Rust

### actix-web
So far I've done the following with `actix-web`:

1. Followed the getting started guide to set up a basic route structure
2. Use env vars for host and port with defaults provided in code from a typed Config struct
3. Created an endpoint that takes Json and an endpoint that uses a path param
   a. `cargo add serde` wasn't enough to let me `derive` `Deserialize` so had to enable feature `derive` for `serde` in `Cargo.toml`
4. Introduced a little file structure to the server
5. Added route with a base64 encoded route param that forwards to a standard route
6. Added sqlite for POC
