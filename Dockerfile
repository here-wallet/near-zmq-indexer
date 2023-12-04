FROM rust:1.71

RUN apt update  -y
RUN apt upgrade  -y

RUN apt install clang libzmq3-dev pkg-config libpq-dev \
    build-essential cargo g++ libclang-dev libssl-dev git  -y

WORKDIR /workdir

COPY . .

EXPOSE 3030
EXPOSE 9555
EXPOSE 24567

RUN cargo build --release


# CMD ["cargo run --release -- -z 9555 --home-dir /near/mainnet run"]
