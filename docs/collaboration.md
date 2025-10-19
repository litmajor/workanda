
# Collaboration Features

This document describes the freelancer-to-freelancer collaboration features in Workanda, including agencies, teams, and joint project bidding.

## Overview

Workanda enables freelancers to collaborate through:
- **Agencies**: Organizations owned by freelancers that manage multiple teams
- **Teams**: Groups of freelancers with complementary skills
- **Joint Bidding**: Teams can bid on larger, complex projects together
- **Revenue Sharing**: Automated distribution of payments among team members
- **Collaborative Workspaces**: Shared project management tools

## Agencies

### Agency Structure

Agencies are freelancer-owned organizations that can:
- Manage multiple teams
- Build reputation as an organization
- Take on enterprise-level projects
- Provide verified status to increase client trust

```rust
pub struct Agency {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,           // Freelancer who owns the agency
    pub team_ids: Vec<Uuid>,      // Teams under this agency
    pub verified: bool,            // Verified status
    pub reputation_score: f64,     // Agency-level reputation
    pub categories: Vec<String>,   // Service categories
    pub projects_completed: i32,   // Total projects completed
    pub avg_delivery_time: i32,    // Average delivery time in days
}
```

### Creating an Agency

Freelancers can create an agency:

```http
POST /api/v1/agencies
Authorization: Bearer <token>

{
  "name": "Tech Solutions Agency",
  "categories": ["Web Development", "Mobile Apps", "UI/UX Design"],
  "description": "Full-stack development agency"
}
```

### Agency Features

1. **Team Management**: Add/remove teams from the agency
2. **Reputation**: Aggregate reputation from all teams and projects
3. **Verification**: Apply for verified status after meeting criteria
4. **Portfolio**: Showcase agency-level work
5. **Branding**: Custom agency profile and branding

## Teams

### Team Structure

Teams are groups of freelancers collaborating together:

```rust
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub leader_id: Uuid,           // Team leader (can be same as agency owner)
    pub member_ids: Vec<Uuid>,     // Team members
    pub skills: Vec<String>,       // Combined team skills
    pub available: bool,           // Availability for new projects
    pub created_at: NaiveDateTime,
}
```

### Team Roles

- **Leader**: Manages team, assigns tasks, distributes revenue
- **Core Members**: Full-time team members with voting rights
- **Contributors**: Part-time or project-based collaborators
- **Specialists**: Brought in for specific skills when needed

### Creating a Team

```http
POST /api/v1/teams
Authorization: Bearer <token>

{
  "name": "Mobile Development Squad",
  "agency_id": "agency-uuid",  // Optional
  "member_ids": ["freelancer1-uuid", "freelancer2-uuid"],
  "skills": ["React Native", "iOS", "Android", "Firebase"],
  "revenue_split": {
    "leader": 30,
    "members": [35, 35]  // Percentages
  }
}
```

## Joint Project Bidding

### How It Works

1. **Project Discovery**: Teams browse projects tagged for team bids
2. **Team Proposal**: Leader submits proposal on behalf of team
3. **Member Approval**: Team members review and approve proposal
4. **Client Selection**: Client selects winning team
5. **Contract Creation**: Smart contract with revenue split defined
6. **Collaborative Work**: Team works together using shared tools
7. **Milestone Completion**: Payments automatically distributed

### Team Proposal Structure

```rust
pub struct TeamProposal {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: i32,
    pub total_bid: f64,
    pub estimated_duration: i32,  // days
    pub team_composition: Vec<TeamMember>,
    pub revenue_split: HashMap<Uuid, f64>,  // freelancer_id -> percentage
    pub deliverables: Vec<String>,
    pub approach: String,         // How team will tackle the project
    pub status: ProposalStatus,
    pub member_approvals: HashMap<Uuid, bool>,
}

pub struct TeamMember {
    pub freelancer_id: Uuid,
    pub role: String,            // "Frontend Dev", "Designer", etc.
    pub responsibilities: Vec<String>,
    pub time_commitment: f64,    // % of time dedicated
}
```

### Submitting Team Proposal

```http
POST /api/v1/proposals/team
Authorization: Bearer <token>

{
  "team_id": "team-uuid",
  "job_id": 123,
  "total_bid": 15000,
  "estimated_duration": 60,
  "team_composition": [
    {
      "freelancer_id": "user1-uuid",
      "role": "Lead Developer",
      "responsibilities": ["Architecture", "Backend API", "Code Review"],
      "time_commitment": 0.6
    },
    {
      "freelancer_id": "user2-uuid",
      "role": "Frontend Developer",
      "responsibilities": ["UI Implementation", "State Management"],
      "time_commitment": 0.5
    }
  ],
  "revenue_split": {
    "user1-uuid": 40,
    "user2-uuid": 35,
    "user3-uuid": 25
  }
}
```

## Revenue Sharing

### Smart Payment Distribution

When milestones are completed, payments automatically split:

```rust
pub struct TeamPayment {
    pub contract_id: u32,
    pub milestone_id: u32,
    pub total_amount: f64,
    pub platform_fee: f64,
    pub agency_cut: Option<f64>,  // If part of agency
    pub distributions: Vec<MemberPayment>,
}

pub struct MemberPayment {
    pub freelancer_id: Uuid,
    pub amount: f64,
    pub percentage: f64,
    pub status: PaymentStatus,
}
```

### Revenue Models

1. **Equal Split**: Everyone gets equal share
2. **Role-Based**: Split based on pre-defined roles
3. **Contribution-Based**: Based on hours worked or tasks completed
4. **Custom**: Negotiated percentages

## Collaborative Tools

### Shared Workspace

Teams get access to:
- **Shared Task Board**: Kanban-style task management
- **Team Chat**: Real-time communication
- **File Sharing**: Centralized document storage
- **Code Repository**: Integrated version control
- **Time Tracking**: Track individual contributions

### Team Dashboard

```rust
pub struct TeamDashboard {
    pub active_projects: Vec<Project>,
    pub total_revenue: f64,
    pub projects_completed: i32,
    pub average_rating: f64,
    pub members: Vec<FreelancerProfile>,
    pub upcoming_milestones: Vec<Milestone>,
}
```

## Project Categories for Teams

### Team-Suitable Projects

Projects that benefit from team collaboration:

- **Large Budget** (>$10,000): Complex projects requiring diverse skills
- **Tight Deadlines**: Multiple people working simultaneously
- **Multi-Disciplinary**: Requires frontend, backend, design, QA
- **Enterprise**: Corporate clients preferring established teams
- **Long-Term**: Ongoing maintenance and development

### Team Project Tags

Clients can tag projects:
- `team_preferred`: Project designed for teams
- `agency_only`: Only verified agencies can bid
- `multi_skill`: Requires 3+ different skill sets
- `enterprise`: Enterprise-level project

## Team Reputation & Trust

### Collective Reputation

Teams build reputation through:
- **Project Success Rate**: % of successfully completed projects
- **Client Ratings**: Average rating across all team projects
- **On-Time Delivery**: % of milestones delivered on time
- **Team Stability**: How long members stay in team
- **Response Time**: How quickly team responds to messages

### Trust Mechanisms

1. **Verified Teams**: Teams with proven track record
2. **Escrow Protection**: All payments held in escrow
3. **Dispute Resolution**: Multi-party dispute handling
4. **Member Vouching**: Team members vouch for each other
5. **Portfolio**: Shared team portfolio with case studies

## API Endpoints

### Agencies

```http
POST   /api/v1/agencies                    # Create agency
GET    /api/v1/agencies                    # List agencies
GET    /api/v1/agencies/{id}               # Get agency details
PUT    /api/v1/agencies/{id}               # Update agency
DELETE /api/v1/agencies/{id}               # Delete agency
POST   /api/v1/agencies/{id}/verify        # Request verification
GET    /api/v1/agencies/{id}/teams         # Get agency teams
POST   /api/v1/agencies/{id}/teams/{team_id}  # Add team to agency
```

### Teams

```http
POST   /api/v1/teams                       # Create team
GET    /api/v1/teams                       # List teams
GET    /api/v1/teams/{id}                  # Get team details
PUT    /api/v1/teams/{id}                  # Update team
DELETE /api/v1/teams/{id}                  # Delete team
POST   /api/v1/teams/{id}/members          # Add member
DELETE /api/v1/teams/{id}/members/{user_id}  # Remove member
GET    /api/v1/teams/{id}/projects         # Get team projects
```

### Team Proposals

```http
POST   /api/v1/proposals/team              # Submit team proposal
GET    /api/v1/proposals/team/{id}         # Get proposal details
PUT    /api/v1/proposals/team/{id}/approve # Member approves proposal
GET    /api/v1/jobs/{id}/team-proposals    # Get team proposals for job
```

### Revenue Distribution

```http
GET    /api/v1/teams/{id}/revenue          # Get revenue breakdown
GET    /api/v1/teams/{id}/payments         # Get payment history
POST   /api/v1/contracts/{id}/distribute   # Trigger payment distribution
```

## Game-Changing Features

### 1. Dynamic Team Formation

AI-powered team matching based on:
- Complementary skills
- Availability alignment
- Previous collaboration success
- Timezone compatibility
- Communication style

### 2. Skill Synergy Scoring

Calculate how well team skills complement each other:
- Gap analysis (what skills are missing)
- Overlap optimization (reduce redundancy)
- Project-skill matching

### 3. Performance Analytics

Track individual and team performance:
- Contribution metrics
- Skill utilization
- Collaboration effectiveness
- Growth tracking

### 4. Team Recommendations

Suggest optimal teams for specific projects based on:
- Historical performance
- Client preferences
- Project requirements
- Success probability

### 5. Revenue Optimization

Smart revenue distribution considering:
- Actual contribution (time + quality)
- Skill rarity and value
- Project criticality
- Market rates

## Best Practices

### For Team Leaders

1. **Clear Roles**: Define responsibilities upfront
2. **Fair Splits**: Transparent revenue distribution
3. **Regular Communication**: Daily standups, weekly reviews
4. **Document Everything**: Decisions, changes, agreements
5. **Recognize Contributions**: Credit individual achievements

### For Team Members

1. **Commitment**: Honor time commitments
2. **Communication**: Proactive updates on progress
3. **Quality**: Maintain high work standards
4. **Collaboration**: Support team members
5. **Flexibility**: Adapt to changing requirements

### For Clients

1. **Clear Requirements**: Detailed project specifications
2. **Single Point of Contact**: Designate team lead
3. **Regular Feedback**: Provide timely input
4. **Milestone-Based**: Break project into clear milestones
5. **Trust the Team**: Allow team autonomy in execution

## Future Enhancements

### Planned Features

- **AI Team Builder**: Automatic team formation based on project
- **Skill Marketplace**: Teams can hire specialists for specific tasks
- **Team Training**: Collaborative learning and upskilling
- **Cross-Team Collaboration**: Multiple teams on mega-projects
- **Team Incubator**: Help freelancers form successful teams
- **Analytics Dashboard**: Deep insights into team performance
- **Client Matching**: Match teams with ideal clients
