FROM rust:1.87

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# CMD ["cargo", "run", "--quiet"]
CMD ["target/release/api-hyper"]
