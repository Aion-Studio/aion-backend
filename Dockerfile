
# Use the latest Rust image as the builder
FROM rust:1.78-bullseye AS builder

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

# Set the working directory
WORKDIR /aionserver

COPY . .
RUN cargo build --release && rm -rf target/debug


####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev curl && rm -rf /var/lib/apt/lists/*

# Copy the specific shared libraries from builder
COPY --from=builder /usr/lib/aarch64-linux-gnu/libssl.so.1.1 /usr/lib/aarch64-linux-gnu/
COPY --from=builder /usr/lib/aarch64-linux-gnu/libcrypto.so.1.1 /usr/lib/aarch64-linux-gnu/

# Import user/group data from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Set the working directory
WORKDIR /aionserver

# Copy the built binary from the builder
COPY --from=builder /aionserver/target/release/aion_server ./

# Use the unprivileged user for running the application
USER aionserver:aionserver

EXPOSE 3000
# Set the command to run your application
CMD ["/aionserver/aion_server"]

