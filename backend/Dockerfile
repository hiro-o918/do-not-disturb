FROM ekidd/rust-musl-builder:1.48.0 as builder

COPY . .

RUN cargo build --release \
  && strip /home/rust/src/target/x86_64-unknown-linux-musl/release/backend

FROM alpine:latest

RUN addgroup -g 1000 app \
  && adduser -D -s /bin/sh -u 1000 -G app app

WORKDIR /usr/local/app/

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/backend /usr/local/app/backend

RUN chown app:app backend

USER app

CMD ["./backend"]
