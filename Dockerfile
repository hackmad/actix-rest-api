# ----------------------------------------------------------------------------
# The build container.
FROM rust:1.62 as build

# Create a new empty shell project.
RUN cd / && cargo new --bin actix-rest-api
WORKDIR /actix-rest-api

# Cache dependencies.
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
RUN cargo install --path . --locked
RUN rm src/*.rs
RUN rm ./target/release/deps/actix_rest_api*

# Build the source.
ADD . ./
RUN cargo install --path . --locked

# ----------------------------------------------------------------------------
# The release container.
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=build /actix-rest-api/target/release/actix-rest-api /
EXPOSE 8000
CMD ["/actix-rest-api"]
