
# Authentication & Security

This document describes the authentication and security mechanisms implemented in Workanda.

## Authentication Methods

### 1. JWT (JSON Web Tokens)

Workanda uses JWT for stateless authentication.

#### Token Structure
```json
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "user_id": "uuid",
    "email": "user@example.com",
    "role": "client|freelancer|admin",
    "exp": 1234567890
  }
}
```

#### Token Lifecycle
- **Expiration**: Tokens expire after 24 hours
- **Refresh**: Users must re-authenticate after expiration
- **Storage**: Store tokens securely (HTTP-only cookies recommended for web apps)

#### Using JWT Tokens

Include the token in the `Authorization` header:
```http
Authorization: Bearer <your_jwt_token>
```

### 2. Multi-Factor Authentication (2FA)

#### TOTP (Time-based One-Time Password)

Workanda supports TOTP-based 2FA using authenticator apps.

**Setup Flow:**
1. User enables 2FA in settings
2. Server generates TOTP secret
3. QR code generated for authenticator app
4. User scans QR code
5. User enters verification code
6. 2FA enabled

**Login with 2FA:**
1. User enters email/password
2. If 2FA enabled, prompt for TOTP code
3. Validate TOTP code
4. Issue JWT token

#### Implementation Details
- Library: `totp-rs`
- Algorithm: HMAC-SHA1
- Time step: 30 seconds
- Code length: 6 digits

### 3. OAuth2

#### Google OAuth Integration

**Authorization Flow:**
```
1. User clicks "Sign in with Google"
2. Redirect to Google OAuth consent screen
3. User authorizes application
4. Google redirects back with authorization code
5. Exchange code for access token
6. Fetch user profile from Google
7. Create/update user account
8. Issue JWT token
```

**Configuration:**
```env
GOOGLE_CLIENT_ID=your_client_id
GOOGLE_CLIENT_SECRET=your_client_secret
GOOGLE_REDIRECT_URI=http://localhost:5000/auth/google/callback
```

## Password Security

### Hashing
- **Algorithm**: bcrypt
- **Work Factor**: 12 (configurable)
- **Salt**: Automatically generated per password

### Password Requirements
- Minimum length: 8 characters
- Must contain: uppercase, lowercase, number, special character
- Cannot contain: username, email

### Password Reset Flow
1. User requests password reset
2. Server generates secure reset token
3. Email sent with reset link
4. User clicks link and enters new password
5. Token validated and password updated
6. All existing sessions invalidated

## Authorization

### Role-Based Access Control (RBAC)

Three primary roles:
- **Client**: Post jobs, hire freelancers
- **Freelancer**: Submit proposals, complete work
- **Admin**: Moderate platform, manage disputes

### Permission Matrix

| Action | Client | Freelancer | Admin |
|--------|--------|------------|-------|
| Post Job | ✅ | ❌ | ✅ |
| Submit Proposal | ❌ | ✅ | ✅ |
| Approve Job | ❌ | ❌ | ✅ |
| Escalate Dispute | ❌ | ❌ | ✅ |
| View All Users | ❌ | ❌ | ✅ |
| Create Contract | ✅ | ❌ | ✅ |
| Release Escrow | ✅ | ❌ | ✅ |

### Middleware Implementation

```rust
// Authentication middleware
pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse, Error> {
    // Extract token from header
    // Validate JWT
    // Attach user info to request
    // Proceed or return 401
}

// Role-based authorization
pub async fn require_role(
    role: &str,
    req: ServiceRequest,
) -> Result<(), Error> {
    // Check user role
    // Allow or return 403
}
```

## Session Management

### Session Storage
- **Development**: In-memory or file-based
- **Production**: Redis for distributed sessions

### Session Security
- Session IDs are cryptographically random
- Sessions expire after inactivity
- Sessions invalidated on password change
- Sessions invalidated on logout

## Security Best Practices

### 1. Password Storage
✅ **Do:**
- Hash passwords with bcrypt
- Use sufficient work factor
- Never store plaintext passwords

❌ **Don't:**
- Store passwords in logs
- Transmit passwords in URLs
- Use weak hashing algorithms (MD5, SHA1)

### 2. Token Security
✅ **Do:**
- Use HTTPS in production
- Set appropriate token expiration
- Validate tokens on every request
- Use secure secret keys

❌ **Don't:**
- Store tokens in localStorage (XSS risk)
- Use predictable token secrets
- Share tokens between users
- Log tokens

### 3. API Security
✅ **Do:**
- Validate all input
- Use parameterized queries (SQLx)
- Implement rate limiting
- Log security events

❌ **Don't:**
- Trust client input
- Use string concatenation for SQL
- Expose sensitive error messages
- Log sensitive data

## Rate Limiting

Protect against brute force attacks:

```rust
// Login endpoint: 5 attempts per 15 minutes
// Password reset: 3 attempts per hour
// API calls: 1000 requests per hour per user
```

## Data Encryption

### At Rest
- Database encryption (PostgreSQL)
- Encrypted backups
- Secure file storage

### In Transit
- HTTPS/TLS for all communications
- Certificate pinning (mobile apps)

### Sensitive Fields
AES-GCM encryption for:
- TOTP secrets
- OAuth tokens
- Payment information

## Security Headers

```
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
X-XSS-Protection: 1; mode=block
Content-Security-Policy: default-src 'self'
```

## Audit Logging

Track security-relevant events:
- Login attempts (success/failure)
- Password changes
- 2FA enable/disable
- Role changes
- Sensitive data access

## Vulnerability Management

### Regular Updates
- Keep dependencies updated
- Monitor security advisories
- Run security audits: `cargo audit`

### Penetration Testing
- Regular security assessments
- Bug bounty program (planned)

## Compliance

- **GDPR**: User data rights, privacy policy
- **PCI-DSS**: Payment card data handling (if applicable)
- **SOC 2**: Security controls and audits (planned)
