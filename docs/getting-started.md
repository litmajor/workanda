
# Getting Started with Workanda

## Prerequisites

- **Rust**: 1.70 or higher
- **PostgreSQL**: 14 or higher
- **Redis**: 6 or higher (optional, for caching)
- **Apache Kafka**: 3.0 or higher (optional, for event streaming)

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/litmajor/workanda.git
cd workanda
```

### 2. Set Up Environment Variables

Create a `.env` file in the project root:

```env
# Database
DATABASE_URL=postgresql://username:password@localhost/workanda

# Redis
REDIS_URL=redis://localhost:6379

# Kafka
KAFKA_BROKERS=localhost:9092

# JWT
JWT_SECRET=your-secret-key-here

# OAuth (Google)
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://localhost:5000/auth/google/callback

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=5000

# Logging
RUST_LOG=info
```

### 3. Set Up the Database

Create the PostgreSQL database:

```bash
createdb workanda
```

Run migrations (if you have migration files):

```bash
# Using SQLx CLI
sqlx migrate run
```

### 4. Build and Run

```bash
# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will start on `http://0.0.0.0:5000`

## Development Setup

### Running in Development Mode

```bash
cargo run
```

### Running Tests

```bash
cargo test --verbose
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Project Structure

```
workanda/
├── src/
│   ├── api/              # API routes and handlers
│   ├── models/           # Data models
│   ├── database/         # Database queries and pool
│   ├── services/         # Business logic services
│   ├── middleware/       # Auth and other middleware
│   ├── auth/             # Authentication logic
│   ├── mfa/              # Multi-factor authentication
│   ├── oauth/            # OAuth providers
│   └── main.rs           # Application entry point
├── migrations/           # Database migrations
├── Cargo.toml            # Rust dependencies
└── .env                  # Environment variables
```

## First Steps

### 1. Create a User Account

```bash
curl -X POST http://localhost:5000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "securepassword123",
    "role": "client"
  }'
```

### 2. Login

```bash
curl -X POST http://localhost:5000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "securepassword123"
  }'
```

### 3. Create a Job (as Client)

```bash
curl -X POST http://localhost:5000/api/v1/jobs \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "title": "Build a website",
    "description": "Need a modern responsive website",
    "budget": 1000,
    "deadline": "2024-12-31T23:59:59Z",
    "category": "Web Development",
    "priority": 1
  }'
```

## Docker Setup (Optional)

If you prefer using Docker:

```bash
# Build the image
docker build -t workanda .

# Run with docker-compose
docker-compose up
```

## Troubleshooting

### Database Connection Issues

- Ensure PostgreSQL is running: `pg_isready`
- Check your `DATABASE_URL` in `.env`
- Verify database exists: `psql -l`

### Port Already in Use

Change the port in `.env`:
```env
SERVER_PORT=5001
```

### Dependencies Not Building

Update Rust and clear cache:
```bash
rustup update
cargo clean
cargo build
```

## Next Steps

- Read the [API Reference](./api-reference.md) for available endpoints
- Explore the [Features](./features.md) documentation
- Understand the [Architecture](./architecture.md)
- Check out [Authentication](./authentication.md) for security details

## Support

For issues or questions:
- Open an issue on [GitHub](https://github.com/litmajor/workanda/issues)
- Check existing documentation
- Review the code examples
