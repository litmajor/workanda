
# Workanda API Reference

Base URL: `http://localhost:5000` (Development)

All API endpoints are versioned under `/api/v1/`

## Authentication

All protected endpoints require a JWT token in the `Authorization` header:
```
Authorization: Bearer <your_jwt_token>
```

### Auth Endpoints

#### Register User
```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "username": "string",
  "email": "string",
  "password": "string",
  "role": "client" | "freelancer"
}
```

**Response:** `201 Created`
```json
{
  "user_id": "uuid",
  "username": "string",
  "email": "string"
}
```

#### Login
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "string",
  "password": "string"
}
```

**Response:** `200 OK`
```json
{
  "token": "jwt_token",
  "user_id": "uuid",
  "role": "string"
}
```

## Jobs

#### Create Job
```http
POST /api/v1/jobs
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "string",
  "description": "string",
  "budget": number,
  "deadline": "ISO8601 datetime",
  "category": "string",
  "priority": number
}
```

**Response:** `201 Created`

#### Get All Jobs
```http
GET /api/v1/jobs
```

**Response:** `200 OK`

#### Get Job by ID
```http
GET /api/v1/jobs/{id}
```

**Response:** `200 OK`

#### Update Job
```http
PUT /api/v1/jobs/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "string",
  "description": "string",
  "budget": number
}
```

**Response:** `200 OK`

#### Delete Job
```http
DELETE /api/v1/jobs/{id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

## Proposals

#### Submit Proposal
```http
POST /api/v1/proposals/{job_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "freelancer_id": number,
  "bid_amount": number,
  "message": "string"
}
```

**Response:** `201 Created`

#### Get Proposals for Job
```http
GET /api/v1/proposals/{job_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Select Proposal
```http
POST /api/v1/proposals/{proposal_id}/select
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Update Proposal
```http
PUT /api/v1/proposals/{proposal_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "bid_amount": number,
  "message": "string"
}
```

**Response:** `200 OK`

#### Delete Proposal
```http
DELETE /api/v1/proposals/{proposal_id}
Authorization: Bearer <token>
```

**Response:** `204 No Content`

## Projects

#### Create Project
```http
POST /api/v1/projects
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "string",
  "budget": number,
  "client_id": "uuid",
  "freelancer_id": "uuid",
  "description": "string",
  "category": "string"
}
```

**Response:** `201 Created`

#### Get All Projects
```http
GET /api/v1/projects
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Get Project by ID
```http
GET /api/v1/projects/{project_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Update Project
```http
PUT /api/v1/projects/{project_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Delete Project
```http
DELETE /api/v1/projects/{project_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

## Contracts

#### Create Contract with Escrow
```http
POST /api/v1/contracts/{contract_id}/escrow
Authorization: Bearer <token>
Content-Type: application/json

{
  "sender_id": number,
  "receiver_id": number,
  "amount": number,
  "currency": "string"
}
```

**Response:** `201 Created`

#### Get Contracts for Client
```http
GET /api/v1/contracts/client/{client_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

## Milestones

#### Create Milestone
```http
POST /api/v1/milestones/{contract_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "string",
  "description": "string",
  "due_date": "ISO8601 datetime",
  "payment_amount": number
}
```

**Response:** `201 Created`

#### Get Milestones
```http
GET /api/v1/milestones/{contract_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Update Milestone
```http
PUT /api/v1/milestones/{contract_id}/{milestone_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Mark Milestone Complete
```http
POST /api/v1/milestones/{contract_id}/{milestone_id}/complete
Authorization: Bearer <token>
```

**Response:** `200 OK`

## Escrow

#### Create Escrow Account
```http
POST /api/v1/escrow/{contract_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "amount": number
}
```

**Response:** `201 Created`

#### Release Escrow
```http
POST /api/v1/escrow/{escrow_id}/release
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Refund Escrow
```http
POST /api/v1/escrow/{escrow_id}/refund
Authorization: Bearer <token>
```

**Response:** `200 OK`

## Payments

#### Create Payment
```http
POST /api/v1/payments/{contract_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "amount": number,
  "status": "string",
  "description": "string"
}
```

**Response:** `201 Created`

#### Get Payments
```http
GET /api/v1/payments/{contract_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

#### Update Payment Status
```http
PUT /api/v1/payments/{payment_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

## Reviews

#### Add Review
```http
POST /api/v1/reviews
Authorization: Bearer <token>
Content-Type: application/json

{
  "freelancer_id": number,
  "client_id": number,
  "rating": number,
  "comment": "string"
}
```

**Response:** `201 Created`

#### Get Reviews for Freelancer
```http
GET /api/v1/reviews/{freelancer_id}
```

**Response:** `200 OK`

#### Get Aggregate Ratings
```http
GET /api/v1/reviews/aggregate/{freelancer_id}
```

**Response:** `200 OK`

## Messages

#### Send Message
```http
POST /api/v1/messages
Authorization: Bearer <token>
Content-Type: application/json

{
  "sender_id": number,
  "receiver_id": number,
  "content": "string"
}
```

**Response:** `201 Created`

#### Get Messages
```http
GET /api/v1/messages/{user_id}
Authorization: Bearer <token>
```

**Response:** `200 OK`

## Admin Endpoints

#### Get All Users
```http
GET /api/v1/admin/users
Authorization: Bearer <admin_token>
```

**Response:** `200 OK`

#### Approve Job
```http
POST /api/v1/admin/approve-job/{job_id}
Authorization: Bearer <admin_token>
```

**Response:** `200 OK`

#### Escalate Dispute
```http
POST /api/v1/admin/escalate-dispute/{dispute_id}
Authorization: Bearer <admin_token>
```

**Response:** `200 OK`

#### Resolve Dispute
```http
POST /api/v1/admin/resolve-dispute/{dispute_id}
Authorization: Bearer <admin_token>
```

**Response:** `200 OK`

## Health Check

```http
GET /health
```

**Response:** `200 OK`
```json
{
  "status": "healthy",
  "timestamp": 1234567890
}
```

## Error Responses

All endpoints may return the following error responses:

- `400 Bad Request`: Invalid request data
- `401 Unauthorized`: Missing or invalid authentication
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error

**Error Format:**
```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```
