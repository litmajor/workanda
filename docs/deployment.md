
# Deployment Guide

This guide covers deploying Workanda on Replit and production best practices.

## Deploying on Replit

### Prerequisites
- Replit account
- PostgreSQL database (use Replit's PostgreSQL)
- Environment variables configured

### Steps

1. **Fork/Import the Repository**
   - Import from GitHub: `https://github.com/litmajor/workanda`
   - Or fork the existing Repl

2. **Configure Environment Variables**
   
   In Replit Secrets (Tools → Secrets):
   ```
   DATABASE_URL=postgresql://user:pass@host:5432/workanda
   JWT_SECRET=your-secret-key
   REDIS_URL=redis://localhost:6379
   RUST_LOG=info
   ```

3. **Set Up the Database**
   
   Use Replit's PostgreSQL tool or shell:
   ```bash
   # Create database
   createdb workanda
   
   # Run migrations (if available)
   sqlx migrate run
   ```

4. **Configure the Run Button**
   
   The `.replit` file should contain:
   ```toml
   run = "cargo run --release"
   
   [nix]
   channel = "stable-24_05"
   
   [deployment]
   run = ["cargo", "run", "--release"]
   deploymentTarget = "cloudrun"
   ```

5. **Test Locally**
   
   Click the Run button to test locally. The server will start on port 5000.

6. **Deploy**
   
   - Click the "Deploy" button in Replit
   - Choose "Autoscale" deployment
   - Configure environment variables for production
   - Deploy!

### Production Configuration

Update `.env` for production:
```env
SERVER_HOST=0.0.0.0
SERVER_PORT=5000
RUST_LOG=warn
DATABASE_URL=<production_database_url>
REDIS_URL=<production_redis_url>
```

## Environment Variables

### Required
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT signing

### Optional
- `REDIS_URL`: Redis connection for caching
- `KAFKA_BROKERS`: Kafka brokers for events
- `GOOGLE_CLIENT_ID`: Google OAuth client ID
- `GOOGLE_CLIENT_SECRET`: Google OAuth secret
- `RUST_LOG`: Logging level (debug, info, warn, error)
- `SERVER_HOST`: Bind address (default: 0.0.0.0)
- `SERVER_PORT`: Port number (default: 5000)

## Database Setup

### PostgreSQL

**Development:**
```bash
# Create database
createdb workanda

# Create user
createuser -P workanda_user

# Grant privileges
psql -c "GRANT ALL PRIVILEGES ON DATABASE workanda TO workanda_user;"
```

**Production:**
- Use managed PostgreSQL (Replit, AWS RDS, etc.)
- Enable SSL connections
- Set up regular backups
- Configure connection pooling

### Migrations

If using SQLx migrations:
```bash
# Create migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Redis Setup (Optional)

**For caching and session management:**

Development:
```bash
redis-server
```

Production:
- Use managed Redis (Replit, AWS ElastiCache, etc.)
- Enable persistence
- Configure memory limits
- Set up replication

## Performance Optimization

### 1. Database
- Add appropriate indexes
- Use connection pooling
- Enable query caching
- Optimize complex queries

### 2. Application
- Enable release mode: `cargo build --release`
- Configure worker threads
- Implement caching strategy
- Use async operations

### 3. Caching
```rust
// Redis caching example
let cached_value = redis.get("key").await?;
if cached_value.is_none() {
    let value = database.query().await?;
    redis.set("key", value, 3600).await?;
}
```

## Monitoring

### Health Checks
```http
GET /health
```

Returns:
```json
{
  "status": "healthy",
  "timestamp": 1234567890
}
```

### Logging

Configure logging levels:
```env
RUST_LOG=workanda=info,actix_web=debug
```

### Metrics (Planned)
- Request count
- Response times
- Error rates
- Database query performance

## Scaling

### Horizontal Scaling
- Deploy multiple instances
- Use load balancer
- Share session state via Redis
- Database connection pooling

### Vertical Scaling
- Increase CPU/memory
- Optimize database
- Enable caching

## Security Checklist

- [ ] Use HTTPS in production
- [ ] Set secure JWT secret
- [ ] Enable database SSL
- [ ] Configure CORS properly
- [ ] Set secure headers
- [ ] Enable rate limiting
- [ ] Use environment variables for secrets
- [ ] Regular security updates
- [ ] Enable audit logging
- [ ] Backup database regularly

## Troubleshooting

### Port Already in Use
```bash
# Find process using port 5000
lsof -i :5000

# Kill process
kill -9 <PID>
```

### Database Connection Failed
- Verify DATABASE_URL is correct
- Check database is running
- Verify network connectivity
- Check firewall rules

### High Memory Usage
- Enable release mode
- Reduce connection pool size
- Implement pagination
- Clear unused cache

## Backup Strategy

### Database
```bash
# Backup
pg_dump workanda > backup.sql

# Restore
psql workanda < backup.sql
```

### Automated Backups
- Daily automated backups
- Retention policy (30 days)
- Test restore procedures
- Store backups securely

## Rollback Procedure

1. Stop current deployment
2. Deploy previous version
3. Restore database if needed
4. Verify functionality
5. Monitor for issues

## Post-Deployment

1. **Verify Health**
   ```bash
   curl https://your-domain.com/health
   ```

2. **Check Logs**
   - Monitor application logs
   - Check error rates
   - Verify expected behavior

3. **Performance Testing**
   - Load testing
   - Stress testing
   - Response time monitoring

4. **Security Scan**
   - Run security audit
   - Check for vulnerabilities
   - Verify SSL/TLS configuration

## Continuous Deployment

Set up GitHub Actions for CI/CD:

```yaml
name: Deploy to Replit

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Deploy to Replit
        # Add deployment steps
```

## Support

For deployment issues:
- Check Replit documentation
- Review application logs
- Contact support team
- Open GitHub issue
