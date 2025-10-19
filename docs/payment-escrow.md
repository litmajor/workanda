
# Payment & Escrow System

This document describes the payment processing and escrow management in Workanda.

## Overview

Workanda uses an escrow-based payment system to protect both clients and freelancers:
- **Clients**: Funds secured until work is completed
- **Freelancers**: Guaranteed payment for completed work
- **Platform**: Mediates disputes and ensures fairness

## Escrow Workflow

### Standard Payment Flow

```
1. Client posts job
2. Freelancer submits proposal
3. Client accepts proposal
4. Contract created
5. Escrow account created
6. Client deposits funds to escrow
7. Freelancer completes work
8. Milestone marked complete
9. Client approves work
10. Funds released from escrow to freelancer
```

### Visual Flow

```
Client Funds → Escrow Account → (Work Completed) → Freelancer
                    ↓
            (Dispute Resolution)
                    ↓
        Refund to Client OR Release to Freelancer
```

## Escrow Account

### Structure
```rust
struct EscrowAccount {
    id: u32,
    contract_id: u32,
    sender_id: u32,        // Client
    receiver_id: u32,      // Freelancer
    amount: f64,
    currency: String,      // USD, EUR, etc.
    status: EscrowStatus,  // pending, released, refunded
    release_conditions: Option<String>,
    created_at: DateTime<Utc>,
    released_at: Option<DateTime<Utc>>,
    refunded_at: Option<DateTime<Utc>>,
}
```

### Escrow States

1. **Pending**: Funds held, work in progress
2. **Released**: Funds transferred to freelancer
3. **Refunded**: Funds returned to client
4. **Disputed**: Under dispute resolution

## Creating Escrow Account

### API Request
```http
POST /api/v1/escrow/{contract_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "amount": 1000.00
}
```

### Response
```json
{
  "id": 123,
  "contract_id": 456,
  "sender_id": 1,
  "receiver_id": 2,
  "amount": 1000.00,
  "currency": "USD",
  "status": "pending",
  "created_at": "2024-01-15T10:00:00Z"
}
```

## Payment Release

### Conditions for Release
- Work completed and submitted
- Milestone marked as complete
- Client approval received
- No active disputes

### Release Process
```http
POST /api/v1/escrow/{escrow_id}/release
Authorization: Bearer <client_token>
```

### Backend Logic
```rust
pub async fn release_escrow(
    pool: &PgPool,
    escrow_id: u32,
) -> Result<(), Error> {
    // 1. Verify escrow exists and is pending
    // 2. Check release conditions
    // 3. Update escrow status to 'released'
    // 4. Transfer funds to freelancer
    // 5. Log transaction
    // 6. Send notifications
    
    sqlx::query(
        "UPDATE escrow_accounts 
         SET status = 'released', released_at = NOW() 
         WHERE id = $1 AND status = 'pending'"
    )
    .bind(escrow_id)
    .execute(pool)
    .await?;
    
    Ok(())
}
```

## Payment Refund

### Conditions for Refund
- Work not started or incomplete
- Client cancellation
- Dispute resolved in favor of client
- Contract violation by freelancer

### Refund Process
```http
POST /api/v1/escrow/{escrow_id}/refund
Authorization: Bearer <authorized_token>
```

### Backend Logic
```rust
pub async fn refund_escrow(
    pool: &PgPool,
    escrow_id: u32,
) -> Result<(), Error> {
    // 1. Verify refund is authorized
    // 2. Update escrow status to 'refunded'
    // 3. Return funds to client
    // 4. Log transaction
    // 5. Send notifications
    
    sqlx::query(
        "UPDATE escrow_accounts 
         SET status = 'refunded', refunded_at = NOW() 
         WHERE id = $1 AND status = 'pending'"
    )
    .bind(escrow_id)
    .execute(pool)
    .await?;
    
    Ok(())
}
```

## Milestone-Based Payments

### Multiple Milestones
```rust
Contract {
    id: 1,
    total_value: 5000,
    milestones: [
        Milestone {
            title: "Phase 1: Design",
            payment_amount: 1500,
            status: "completed"
        },
        Milestone {
            title: "Phase 2: Development",
            payment_amount: 2500,
            status: "in_progress"
        },
        Milestone {
            title: "Phase 3: Testing",
            payment_amount: 1000,
            status: "pending"
        }
    ]
}
```

### Partial Releases
- Funds released per milestone
- Reduces risk for both parties
- Provides progress checkpoints

## Dispute Resolution

### Dispute Levels

1. **Initial Review**: Platform reviews evidence
2. **Mediation**: Facilitator helps parties reach agreement
3. **Arbitration**: Platform makes binding decision
4. **Resolved**: Final outcome implemented

### Dispute Flow

```
Dispute Filed → Initial Review → Mediation → Arbitration → Resolved
                     ↓              ↓            ↓
               Can resolve at any stage
```

### Filing a Dispute
```http
POST /api/v1/escrow/dispute/{escrow_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "reason": "Work not completed as agreed",
  "evidence": ["url1", "url2"]
}
```

### Dispute Resolution
```rust
enum DisputeResolution {
    Refund,          // Full refund to client
    Release,         // Full release to freelancer
    PartialRefund(f64),  // Partial refund
    PartialRelease(f64), // Partial release
}
```

### Escalation
```http
POST /api/v1/admin/escalate-dispute/{dispute_id}
Authorization: Bearer <admin_token>
```

Escalates dispute to next level (Mediation → Arbitration)

## Payment Reminders

### Automated Reminders
- Sent when payment is due
- Configurable frequency
- Email and in-app notifications

### Implementation
```rust
pub async fn send_payment_reminders(
    pool: &PgPool,
) -> Result<(), Error> {
    // 1. Find overdue payments
    // 2. Check last reminder date
    // 3. Send reminder email
    // 4. Update reminder log
    
    let overdue = sqlx::query_as::<_, Payment>(
        "SELECT * FROM payments 
         WHERE status = 'pending' 
         AND due_date < NOW()"
    )
    .fetch_all(pool)
    .await?;
    
    for payment in overdue {
        send_reminder_email(&payment).await?;
    }
    
    Ok(())
}
```

## Payment Methods

### Supported Methods
- Credit/Debit Cards (planned)
- Bank Transfers (planned)
- Cryptocurrency (planned)
- PayPal (planned)

### Multi-Currency Support
```rust
enum Currency {
    USD,
    EUR,
    GBP,
    BTC,  // Cryptocurrency support
    ETH,
}
```

## Fee Structure

### Platform Fees

| Membership Tier | Fee Percentage |
|----------------|----------------|
| Basic          | 15%            |
| Normal         | 10%            |
| Premium        | 7%             |
| Enterprise     | 5%             |

### Fee Calculation
```rust
pub fn calculate_platform_fee(
    amount: f64,
    membership_tier: MembershipTier
) -> f64 {
    let fee_percentage = match membership_tier {
        MembershipTier::Basic => 0.15,
        MembershipTier::Normal => 0.10,
        MembershipTier::Premium => 0.07,
        MembershipTier::Enterprise => 0.05,
    };
    
    amount * fee_percentage
}
```

## Transaction History

### Payment Records
All transactions are logged for audit:
```rust
struct PaymentHistory {
    id: u32,
    contract_id: u32,
    amount: f64,
    fee: f64,
    status: String,
    payment_method: String,
    transaction_id: String,
    created_at: DateTime<Utc>,
}
```

### View History
```http
GET /api/v1/payments/{contract_id}
Authorization: Bearer <token>
```

## Security Measures

### Payment Security
- PCI-DSS compliance (when handling cards)
- Encrypted data transmission
- Secure payment gateway integration
- Fraud detection

### Escrow Security
- Funds held in secure accounts
- Multi-signature withdrawals (planned)
- Regular audits
- Insurance coverage (planned)

## Best Practices

### For Clients
1. Fund escrow before work starts
2. Define clear milestones
3. Review work promptly
4. Release payments on time
5. Use dispute resolution if needed

### For Freelancers
1. Confirm escrow funded before starting
2. Complete milestones as agreed
3. Submit quality work
4. Provide regular updates
5. Document all deliverables

### For Platform
1. Monitor all transactions
2. Detect suspicious activity
3. Respond to disputes quickly
4. Maintain escrow integrity
5. Provide transparency
