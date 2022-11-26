FROM rust:latest as build
WORKDIR /usr/src/hello-world
COPY hello-world .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/hello-world /hello-world
CMD ["/hello-world"]
