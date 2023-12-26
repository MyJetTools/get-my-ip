FROM ubuntu:22.04
COPY ./target/release/get-my-ip ./target/release/get-my-ip
ENTRYPOINT ["./target/release/get-my-ip"]