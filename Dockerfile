FROM rust:1.62
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
COPY . .

ARG RUST_ENV
RUN if [ "$RUST_ENV" === "production" ]; then cargo build --release; else cargo build; fi

CMD [ "trunk", "serve", "--release" ]