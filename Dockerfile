# latest rust release as base image
FROM rust:1.95.0

# create/cd into app folder
WORKDIR /app

# install required system dependencies
RUN apt update && apt install lld clang -y

# copy files from working environment to our docker image
COPY . .

# to handle sqlx, force it to look at saved metadata instead of trying to query live databse
ENV SQLX_OFFLINE=true

# build the binary
RUN cargo build --release
ENV APP_ENVIRONMENT=production

# launch the binary
ENTRYPOINT [ "./target/release/emailnewsletter" ]
