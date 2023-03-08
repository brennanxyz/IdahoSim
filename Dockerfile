FROM osgeo/proj:latest

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Install cargo
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y cargo pkg-config proj-bin cmake g++ sqlite3 libsqlite3-dev libtiff-dev curl libcurl4-openssl-dev libssl-dev libclang-dev

# Set working directory
WORKDIR /usr/src/app

# Copy source code
COPY . .

# Build
RUN cargo build --release

# Run
CMD ["cargo", "run", "--release"]