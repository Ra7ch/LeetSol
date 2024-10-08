FROM rust:1.72-slim

WORKDIR /usr/src/app

RUN mkdir uploads/

COPY ./rust_audit_service/ .

RUN cargo build --release

CMD ["./target/release/rust_audit_service"]
