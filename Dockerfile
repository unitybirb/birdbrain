FROM rust as builder
WORKDIR birdbrain
COPY . .
RUN cargo build --release --bin birdbrain

FROM rust as runtime
WORKDIR birdbrain
COPY --from=builder /birdbrain/target/release/birdbrain /usr/local/bin
ENTRYPOINT ["./usr/local/bin/birdbrain"]