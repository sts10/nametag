FROM rust:1-alpine3.12 as build

# alpine needs this, otherwise we get an error when compiling rand v0.7.3
RUN apk add --no-cache musl-dev

RUN USER=root cargo new --bin nametag

WORKDIR /nametag

# copy manifests
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

# cache your dependencies
RUN cargo build --release \
  && rm src/*.rs

# copy source tree
COPY . .

# build for release
RUN rm -f target/release/deps/nametag \
  && cargo build --release \
  && cargo install --path .

# runner stage to decrease image size
FROM alpine:latest

# copy the build artifact from the build stage
COPY --from=build /nametag/target/release/nametag .

# create user/group to run instead of sudo
RUN addgroup -g 1000 nametaggroup \
  && adduser -D -s /bin/sh -u 1000 -G nametaggroup nametaguser \
  && chown nametaguser:nametaggroup /nametag

USER nametaguser

CMD ["/bin/sh"]
