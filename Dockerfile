
# Use an official Rust runtime as a parent image
FROM rust:1.67

# Set the working directory in the container to /idle_rpg
WORKDIR /idle_rpg

# Copy the current directory contents into the container at /idle_rpg
COPY . /idle_rpg

# Install dependencies using cargo
RUN cargo install --path .

# Make port 8000 available to the world outside this container
EXPOSE 8000

# Define environment variable
ENV RUST_LOG info
# Run the binary program produced by `cargo install`
CMD ["idle_rpg"]
