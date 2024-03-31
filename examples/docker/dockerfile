# Use Alpine Linux as the base image
FROM rust:latest

# Install system dependencies
RUN apt-get update -y

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the project
RUN cargo build --release

# Expose port 8080 (example)
EXPOSE 8080

# Define the command to run the project (generic)
CMD ["cargo", "run", "--release"]
