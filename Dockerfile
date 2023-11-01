# Use an official Rust runtime as a parent image
FROM rust:1.72.0 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Install dependencies and build the project
RUN cargo build --release

# Expose port 3000 to the outside world
EXPOSE 3030

# Define environment variable
ENV APP_PORT 3030

# Run the binary program produced by `cargo install`
CMD ["./target/release/rust_api"]
