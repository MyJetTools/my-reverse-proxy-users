FROM ubuntu:22.04
COPY ./target/release/my-reverse-proxy-users ./target/release/my-reverse-proxy-users
ENTRYPOINT ["./target/release/my-reverse-proxy-users"]