FROM liuchong/rustup

RUN groupadd -r user --gid="1000" \
    && adduser --home "/home/user" --gid "1000" --disabled-password --disabled-login --gecos '' --shell "/bin/bash" --uid "1000" user \
    && chown -R user /home/user

WORKDIR /app
COPY docker-entrypoint.sh /docker-entrypoint.sh
EXPOSE 8080
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD []
