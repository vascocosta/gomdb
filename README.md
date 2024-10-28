# GOMDb

Simple web interface to the Open Movie Database API.

This project was mostly intended as a simple showcase of how to implement a minimalistic rust web app that is essentially server side rendered in the backend, albeit still exhibiting client side reactivity, without explicit use of javascript. To achieve this goal, the following software stack was found to be extremely elegant to use:

* [htmx](https://htmx.org/)
* [axum](https://github.com/tokio-rs/axum)
* [maud](https://maud.lambda.xyz/)

## Screenshots

![Main](https://i.imgur.com/SJN6C92.png)

## Deploy

### Build

To build `gomdb` you need the `Rust toolchain` as well as these `dependencies`:

* anyhow = "1.0.91"
* axum = "0.7.7"
* maud = { version = "0.26.0", features = ["axum"] }
* rand = "0.8.5"
* reqwest = { version = "0.12.8", features = ["json"] }
* serde = { version = "1.0.210", features = ["derive"] }
* serde_json = "1.0.132"
* tokio = { version = "1.40.0", features = ["full"] }
* tower-http = { version = "0.6.1", features = ["full"] }

Follow these steps to fetch and compile the source of `gomdb` and its `dependencies`:

```
git clone https://github.com/vascocosta/gomdb.git
cd gomdb
cargo build --release
```

### Run

Finally you can run the app with:

```
OMDB_KEY="YOUR_OMDB_API_KEY" ./target/release/gomdb &
```