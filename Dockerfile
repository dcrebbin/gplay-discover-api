# Use an official Rust image as a builder
FROM rust:1.75 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin gplay_discover_api
WORKDIR /gplay_discover_api

# Copy the Cargo.toml and Cargo.lock files and build dependencies
COPY . .
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code and build the application
COPY ./src ./src
RUN rm ./target/release/deps/gplay_discover_api*
RUN cargo build --release

# Use the same Rust base image for the final image
FROM rust:1.75
COPY --from=builder /gplay_discover_api/target/release/gplay_discover_api /usr/local/bin/gplay_discover_api
EXPOSE 443 80
CMD ["gplay_discover_api"]

