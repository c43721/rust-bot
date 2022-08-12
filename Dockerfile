FROM ekidd/rust-musl-builder AS build

ADD . ./
RUN sudo chown -R rust:rust .

RUN cargo build --release

FROM alpine:latest

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-bot /
ENV RUST_LOG=info
CMD ["/rust-bot"]