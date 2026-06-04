ARG RUST_VERSION
FROM rust:${RUST_VERSION}

ARG UID
ARG GID
ARG USER
ARG PASSWORD
ARG SHELL

RUN apt -y update && apt -y upgrade && apt install -y nano vim curl sudo && \
    groupadd --gid $GID $USER && \
    useradd --uid $UID --gid $GID --shell $SHELL --home-dir /home/$USER --create-home $USER && \
    echo "$USER:$PASSWORD" | chpasswd

USER $USER
WORKDIR /app
