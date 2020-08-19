FROM rust:1.45.2 AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Use dummy project to download deps.
# If deps haven't changed, docker cache lets us skip download.
RUN USER=root cargo new json_echo
WORKDIR /usr/src/json_echo
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/json_echo .
USER 1000
CMD ["./json_echo"]