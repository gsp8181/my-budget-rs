FROM clux/muslrust:stable as build

# create a new empty shell project
RUN USER=root cargo new --bin my-budget
WORKDIR /my-budget

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release

COPY ./Rocket.toml .
COPY ./wwwroot ./wwwroot

RUN cp $(find target/* -type d -not -name 'release' -print -quit)/release/my-budget my-budget

# our final base
FROM alpine

WORKDIR /my-budget

# copy the build artifact from the build stage
COPY --from=build /my-budget/my-budget .
COPY --from=build /my-budget/migrations ./migrations
COPY --from=build /my-budget/Rocket.toml .
COPY --from=build /my-budget/wwwroot ./wwwroot

# set the startup command to run your binary
CMD ["./my-budget"]

VOLUME ["/my-budget/storage"]
EXPOSE 8000
