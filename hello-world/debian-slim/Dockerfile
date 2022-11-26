FROM rust:latest as build
WORKDIR /usr/src/hello-world
COPY hello-world .
RUN cargo install --path .

FROM debian:stable-slim
COPY --from=build /usr/local/cargo/bin/hello-world /usr/local/bin/hello-world
CMD ["hello-world"]
