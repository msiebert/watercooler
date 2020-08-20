FROM rustlang/rust:nightly

RUN groupadd -r user --gid="1000" \
    && adduser --home "/home/user" --gid "1000" --disabled-password --disabled-login --gecos '' --shell "/bin/bash" --uid "1000" user \
    && chown -R user /home/user

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo install cargo-watch

EXPOSE 8080
