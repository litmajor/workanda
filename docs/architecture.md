
# Workanda Architecture

## System Overview

Workanda is built using a modular, service-oriented architecture with Rust as the primary backend language, PostgreSQL for data persistence, Redis for caching, and Apache Kafka for event streaming.

## High-Level Architecture

```
┌─────────────────┐
│   Client Apps   │
│  (Web/Mobile)   │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────┐
│         API Gateway                 │
│      (Actix-web Router)             │
└────────┬───────────────────────┬────┘
         │                       │
         ▼                       ▼
┌────────────────┐     ┌──────────────────┐
│  Auth Service  │     │  Core Services   │
│  - JWT         │     │  - User Mgmt     │
│  - OAuth2      │     │  - Projects      │
│  - 2FA/TOTP    │     │  - Jobs          │
└────────┬───────┘     │  - Payments      │
         │             │  - Escrow        │
         │             │  - Reviews       │
         │             └────────┬─────────┘
         │                      │
         ▼                      ▼
┌─────────────────────────────────────┐
│        Database Layer               │
│  ┌──────────┐  ┌──────────────┐    │
│  │PostgreSQL│  │    Redis     │    │
│  │ (Primary)│  │  (Caching)   │    │
│  └──────────┘  └──────────────┘    │
└─────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────┐
│      Message Queue (Kafka)          │
│  - Events                           │
│  - Notifications                    │
│  - Analytics                        │
└─────────────────────────────────────┘
```

## Core Components

### 1. API Layer (`src/api/v1/`)

The API layer is organized into versioned modules, currently supporting v1:

- **Authentication** (`auth/`): Login, registration, OAuth
- **User Management** (`user/`, `freelancer/`, `client/`): Profile management
- **Jobs** (`job/`): Job posting and management
- **Proposals** (`proposal/`): Bid submission and selection
- **Projects** (`project/`): Project lifecycle management
- **Contracts** (`contracts/`): Contract creation and management
- **Milestones** (`milestones/`): Milestone tracking
- **Payments** (`payments/`): Payment processing
- **Escrow** (`escrow/`): Escrow account management
- **Reviews** (`reviews/`): Rating and feedback system
- **Messages** (`messages/`): Direct messaging
- **Admin** (`admin/`): Administrative functions

### 2. Service Layer

#### User Management Service (`src/user-management/`)
- User authentication and authorization
- Profile management for clients and freelancers
- KYC verification
- Password management
- Role and privilege management

#### Project Service (`src/project-service/`)
- Project creation and lifecycle management
- Project status transitions
- Task management within projects
- Workflow orchestration

#### Escrow Service (`src/services/escrow_service.rs`)
- Escrow account creation
- Fund management
- Dispute handling
- Payment release/refund logic

### 3. Data Layer

#### Models (`src/models/`)
Comprehensive data models for all entities:
- Users (clients, freelancers, admins)
- Jobs and proposals
- Projects and tasks
- Contracts and milestones
- Payments and escrow accounts
- Reviews and ratings
- Messages and notifications

#### Database (`src/database/`)
- Connection pool management
- Query abstraction layer
- Migration scripts
- Database utilities

### 4. Middleware Layer (`src/middleware/`)

- **Authentication Middleware**: JWT validation
- **Authorization**: Role-based access control
- **Rate Limiting**: Request throttling
- **Logging**: Request/response logging
- **Error Handling**: Centralized error processing

## Security Architecture

### Authentication Flow

```
User Request → Auth Middleware → JWT Validation
                                      ↓
                                 Valid Token?
                                   ✓  │  ✗
                          ┌──────────┴──────────┐
                          ▼                     ▼
                    Extract User ID      401 Unauthorized
                          ↓
                    Attach to Request
                          ↓
                    Route Handler
```

### Multi-Factor Authentication

1. User enters credentials
2. Primary authentication (password)
3. TOTP token generation/validation
4. JWT issuance upon successful 2FA

### OAuth2 Flow

1. User initiates OAuth login
2. Redirect to provider (Google)
3. Callback with authorization code
4. Exchange code for access token
5. Fetch user profile
6. Create/update user account
7. Issue JWT

## Data Flow

### Job Posting to Completion

```
Client Creates Job → Job Posted
                         ↓
                  Freelancers Submit Proposals
                         ↓
                  Client Selects Proposal
                         ↓
                  Contract Created with Escrow
                         ↓
                  Milestones Defined
                         ↓
                  Work Begins
                         ↓
                  Milestone Completed
                         ↓
                  Client Approves
                         ↓
                  Payment Released from Escrow
                         ↓
                  Review & Rating
```

## Database Schema

### Core Tables

- **users**: User accounts and authentication
- **clients**: Client-specific profile data
- **freelancers**: Freelancer-specific profile data
- **jobs**: Job postings
- **proposals**: Freelancer bids
- **projects**: Project management
- **contracts**: Work agreements
- **contract_milestones**: Project milestones
- **escrow_accounts**: Escrow fund management
- **payments**: Payment transactions
- **reviews**: Ratings and feedback
- **messages**: Direct messaging

### Relationships

- User (1) → (Many) Jobs (as client)
- Job (1) → (Many) Proposals
- User (1) → (Many) Proposals (as freelancer)
- Proposal (1) → (1) Contract
- Contract (1) → (1) Escrow Account
- Contract (1) → (Many) Milestones
- Contract (1) → (Many) Payments
- User (2) → (Many) Messages (sender/receiver)

## Technology Stack Details

### Backend Framework
- **Actix-web**: High-performance async web framework
- **Actix-actors**: Actor model for concurrency
- **Tokio**: Async runtime

### Database & Caching
- **SQLx**: Compile-time verified SQL queries
- **PostgreSQL**: Primary data store
- **Redis**: Session and cache management

### Authentication & Security
- **jsonwebtoken**: JWT implementation
- **bcrypt**: Password hashing
- **oauth2**: OAuth2 client
- **totp-rs**: TOTP 2FA
- **aes-gcm**: Data encryption

### Messaging
- **rdkafka**: Kafka client for event streaming

### Utilities
- **serde**: Serialization/deserialization
- **chrono**: Date/time handling
- **uuid**: Unique identifier generation
- **dotenv**: Environment configuration

## Deployment Architecture

```
┌─────────────────────────────────────┐
│         Load Balancer               │
└────────┬────────────────────────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌────────┐ ┌────────┐
│ App 1  │ │ App 2  │  (Rust services)
└───┬────┘ └───┬────┘
    │          │
    └────┬─────┘
         ▼
┌─────────────────┐
│   PostgreSQL    │
│   (Primary)     │
└─────────────────┘
         │
         ▼
┌─────────────────┐
│   PostgreSQL    │
│   (Replica)     │
└─────────────────┘
```

## Scalability Considerations

1. **Horizontal Scaling**: Stateless API servers
2. **Database Replication**: Read replicas for query load
3. **Caching Strategy**: Redis for frequently accessed data
4. **Event-Driven Architecture**: Kafka for async processing
5. **Connection Pooling**: Efficient database connections

## Monitoring & Observability

- **Logging**: Structured logging with `env_logger`
- **Metrics**: Application performance metrics (planned)
- **Health Checks**: `/health` endpoint for service monitoring
- **Error Tracking**: Centralized error handling and reporting
