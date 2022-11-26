FROM rust:alpine as build
WORKDIR /usr/src/hello-world
COPY hello-world .
RUN cargo install --path .

FROM alpine
COPY --from=build /usr/local/cargo/bin/hello-world /hello-world
CMD ["/hello-world"]
