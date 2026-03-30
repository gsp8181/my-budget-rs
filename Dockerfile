# Build frontend (nextgen-client) to produce static `build/` output
FROM node:18 AS node-build
WORKDIR /app
COPY nextgen-client/package*.json ./nextgen-client/
ENV NODE_ENV=production
RUN cd nextgen-client && npm ci --omit=dev --silent
COPY nextgen-client ./nextgen-client
RUN cd nextgen-client && npm run build

# Build Rust binary (musl)
FROM clux/muslrust:stable AS build

# create a new empty shell project
RUN USER=root cargo new --bin my-budget
WORKDIR /my-budget

# copy over your manifests and sources
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release

COPY ./Rocket.toml .
# copy frontend build output from node-build stage into wwwroot
COPY --from=node-build /app/nextgen-client/build ./wwwroot

# copy the compiled binary out of the target directory
#RUN cp $(find target/* -type d -not -name 'release' -print -quit)/release/my-budget my-budget
RUN cp $(for i in $(ls -d target/*/); do echo ${i%%/}; done | grep linux)/release/my-budget my-budget

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
