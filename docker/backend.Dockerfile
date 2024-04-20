FROM rust:alpine3.19

WORKDIR /app

COPY ./backend/src  ./src
COPY ./backend/database ./database
COPY ./backend/Cargo.toml ./backend/Cargo.lock ./

RUN apk add --no-cache musl-dev && cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]
