# Multi-stage Dockerfile for Helios
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Set working directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./
COPY helios-core/Cargo.toml helios-core/
COPY helios-leptos/Cargo.toml helios-leptos/
COPY helios-macros/Cargo.toml helios-macros/
COPY helios-wasm/Cargo.toml helios-wasm/
COPY helios-wasm-core/Cargo.toml helios-wasm-core/
COPY helios-examples/Cargo.toml helios-examples/

# Build dependencies
RUN cargo build --release

# Copy source code
COPY . .

# Build the project
RUN cargo build --release

# Build WASM packages
RUN cd helios-wasm-core && wasm-pack build --target web --out-dir pkg
RUN cd helios-wasm && wasm-pack build --target web --out-dir pkg

# Production stage
FROM nginx:alpine

# Install Node.js for serving
RUN apk add --no-cache nodejs npm

# Copy built artifacts
COPY --from=builder /app/target/release/ /usr/local/bin/
COPY --from=builder /app/helios-wasm-core/pkg/ /usr/share/nginx/html/wasm/
COPY --from=builder /app/helios-wasm/pkg/ /usr/share/nginx/html/wasm/
COPY --from=builder /app/docs/ /usr/share/nginx/html/docs/
COPY --from=builder /app/examples/ /usr/share/nginx/html/examples/

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy package.json for npm serving
COPY package.json /usr/share/nginx/html/

# Expose ports
EXPOSE 80 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
