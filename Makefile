all:
	make -B \
	hello-world-alpine \
	hello-world-debian \
	hello-world-debian-slim \
	hello-world-distroless-cc \
	hello-world-distroless-static \
	hello-world-scratch-glibc \
	hello-world-scratch-musl

hello-world-alpine:
	docker build --platform linux/amd64 -t rust-docker-hello-world:alpine -f hello-world/alpine/Dockerfile .

hello-world-debian:
	docker build --platform linux/amd64 -t rust-docker-hello-world:debian -f hello-world/debian/Dockerfile .

hello-world-debian-slim:
	docker build --platform linux/amd64 -t rust-docker-hello-world:debian-slim -f hello-world/debian-slim/Dockerfile .

hello-world-distroless-cc:
	docker build --platform linux/amd64 -t rust-docker-hello-world:distroless-cc -f hello-world/distroless-cc/Dockerfile .

hello-world-distroless-static:
	docker build --platform linux/amd64 -t rust-docker-hello-world:distroless-static -f hello-world/distroless-static/Dockerfile .

hello-world-scratch-glibc:
	docker build --platform linux/amd64 -t rust-docker-hello-world:scratch-glibc -f hello-world/scratch-glibc/Dockerfile .

hello-world-scratch-musl:
	docker build --platform linux/amd64 -t rust-docker-hello-world:scratch-musl -f hello-world/scratch-musl/Dockerfile .

webserver-scratch-glibc:
	docker build --platform linux/amd64 -t rust-docker-webserver:scratch-glibc -f webserver/scratch-glibc/Dockerfile .
