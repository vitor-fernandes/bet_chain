FROM rust:1.81.0

COPY . /opt/bet_chain/

WORKDIR /opt/bet_chain/

RUN apt update && apt install libclang-dev -y

#RUN cargo build --release
