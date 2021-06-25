FROM rust:1.53.0

ADD ./ /source

WORKDIR /source

RUN cargo update

RUN cargo build --release

CMD ./target/release/rest-api