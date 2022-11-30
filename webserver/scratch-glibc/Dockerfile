FROM rust:latest as build
WORKDIR /usr/src/webserver
COPY webserver .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --target x86_64-unknown-linux-gnu --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/webserver /webserver
CMD ["/webserver"]
