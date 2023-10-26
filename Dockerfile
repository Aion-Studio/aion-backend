## Use an official Rust runtime as a parent image
#FROM rust:1.67
#
## Set the working directory in the container to /idle_rpg
#WORKDIR /idle_rpg
#
## Copy the current directory contents into the container at /idle_rpg
#COPY . /idle_rpg
#
## Install dependencies using cargo
#RUN cargo install --path .
#
## Make port 8000 available to the world outside this container
#EXPOSE 8000
#
## Define environment variable
#ENV RUST_LOG info
## Run the binary program produced by `cargo install`
#CMD ["idle_rpg"]

# Use the latest Rust image as the builder
FROM rust:latest AS builder

# Update certificates
RUN update-ca-certificates

# Create appuser for your application
ENV USER=aionserver
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /aionserver

COPY ./ .

# Build the application
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

# Import user/group data from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder
COPY --from=builder /aionserver/target/release/aion_server ./

# Use the unprivileged user for running the application
USER aionserver:aionserver

# Set the command to run your application
CMD ["/app/aion_server"]
