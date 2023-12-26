FROM ubuntu:22.04
COPY ./target/release/get_my_ip ./target/release/get_my_ip
ENTRYPOINT ["./target/release/get_my_ip"]