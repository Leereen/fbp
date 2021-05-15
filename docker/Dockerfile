# Build images
FROM rust:1.43 as builder

# dependencies "cache"
RUN USER=root cargo new --bin fbp
WORKDIR ./fbp
RUN rustup default nightly
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# now building FBP proper
ADD . ./
RUN rm ./target/release/deps/fbp*

RUN cargo build --release
RUN sed -i "s@secret_key.*@secret_key = \"$(openssl rand -base64 32)\"@g" Rocket.toml

# ----------------
# Deployment image

FROM debian:buster
ARG APP=/usr/src/app

EXPOSE 8000

ENV APP_USER=appuser

RUN adduser --no-create-home --disabled-password ${APP_USER} \
    && mkdir -p ${APP}
RUN apt-get update && apt-get install sqlite3 -yf

COPY --from=builder /fbp/target/release/fbp ${APP}/fbp
COPY --from=builder /fbp/Rocket.toml ${APP}/Rocket.toml

RUN chown -R $APP_USER: ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./fbp"]