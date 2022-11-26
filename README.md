# Rust Docker Cheatsheet

Notes on building and running [Rust](https://www.rust-lang.org/) binaries in [Docker](https://www.docker.com/).

## Programs

### Hello World

Image information for the Rust [hello world](./hello-world/)[^1] program.

| Base | Size | Libc | CRT Linkage |
| - | - | - | - |
| debian | 124MB | glibc | dynamic |
| debian:slim | 80.8MB | glibc | dynamic |
| distroless:cc | 23MB | glibc | dynamic |
| alpine | 7.47MB | musl | static |
| distroless:static | 3.63MB | glibc | static |
| scratch | 1.29MB | glibc | static |
| scratch | 416KB | musl | static |

## Base Images

### Alpine

[Alpine](https://www.alpinelinux.org/about/) provides the smallest OS while keeping just about everything a Rust program will need to function.

The Rust official Docker support Alpine for building Rust programs through the [`:alpine`](https://hub.docker.com/_/rust/tags?page=1&name=alpine) tag.

It's important to note that Alpine uses [musl](https://musl.libc.org/) libc instead of glibc, and as such you may encounter issues with other libraries expecting the glibc implementaiton, and potentially servere performance differences. The Rust std defaults to using the [system allocator](https://doc.rust-lang.org/std/alloc/struct.System.html), in this instance the malloc implementation provided by musl, however it can be replaced with another allocator by using the [global_allocator](https://doc.rust-lang.org/std/alloc/index.html) attribute.

### Debian

[Debian](https://en.wikipedia.org/wiki/Debian) provides a stable base from which to run any Rust program, and is the default used by the Rust official Docker images.

While providing the most stable and predictable environment, Debian also includes many features, programs, and libraries which are unnecessary for most workloads.

Using the `debian:slim` tag will shave 40MB off, taking it from 120MB to 80MB, though you'll need to reinstall `ca-certificates` for typical networked applications.

### Distroless

[Distroless](https://github.com/GoogleContainerTools/distroless) is an initiative to provide containers with only what is needed to run a given application.

There are a number of flavours provided, however [cc](https://github.com/GoogleContainerTools/distroless/blob/main/cc/README.md) and [static](https://github.com/GoogleContainerTools/distroless/blob/main/base/README.md) are most relevent for Rust, and are very slim, coming in at 22MB and 3MB respectively.

### Scratch

[Scratch](https://hub.docker.com/_/scratch), while not technically an image, does provide the means to achieve the smallest image sizes.

Because scratch includes nothing, you will need to provide _everything_ your application needs to operate, including libc, ssl libraries, certificates, etc.

## Concepts

### Libc, glibc, and musl

Rust "the language" has no dependency on libc (or any c runtime), however, the Rust standard library ([`std`](https://doc.rust-lang.org/std/)) does[^2] to provide a convenient and stable way to interact with the host system.

Because Rust programs usually want to use the std, and libc is usually provided by the OS, the base image is an important factor for containerised Rust.

While [glibc](https://www.gnu.org/software/libc/) is the "default" libc, [musl](https://musl.libc.org/) is also supported.

Care needs to be taken when choosing to use musl as other linked libraries may not be expecting it, it can have [surprising performance implications](https://www.linkedin.com/pulse/testing-alternative-c-memory-allocators-pt-2-musl-mystery-gomes), and it has relatively less development resources.

### C runtime linkage

The Rust standard library supports both static and dynamic [linking of the C runtime](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) (CRT). Generally it will be dynamically linked, except for targets specifying musl.

## Rust Binary Size

If you're looking to reduce the size of your images and you've already optimised the base image, reducing the size of your Rust binary is a good next step.

There are many factors that affect binary size in Rust, I recommend you start with [min-sized-rust](https://github.com/johnthagen/min-sized-rust).

---


[^1]: Symbol [stripping](https://doc.rust-lang.org/cargo/reference/profiles.html#strip) is enabled.

[^2]: Use of the std can be disabled with the [no_std](https://docs.rust-embedded.org/book/intro/no-std.html#summary) attribute, in which case the [`core`](https://doc.rust-lang.org/core/) library will still be available.

There are also experimental ways to write rust code with the convenience of std without needing libc/C code, such as [rustix](https://github.com/bytecodealliance/rustix) + [mustang](https://github.com/sunfishcode/mustang) or [relibc](https://gitlab.redox-os.org/redox-os/relibc/).