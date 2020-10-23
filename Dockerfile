# Use a build image to create the binary to run
FROM rust:1.47 as build

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./static ./static
COPY ./templates ./templates
COPY ./content ./content

RUN apt-get update && apt-get install -y clang

RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /target/release/pulfer-dev .
COPY --from=build /static ./static
COPY --from=build /templates ./templates

# set the startup command to run your binary
CMD ["./pulfer-dev"]