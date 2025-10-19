
# Project Management

This document describes the project and task management features in Workanda.

## Overview

Workanda provides comprehensive project management capabilities including:
- Project lifecycle management
- Task tracking
- Milestone management
- Team collaboration
- Workflow automation
- Time tracking

## Project Lifecycle

### Project States

1. **Initiated**: Project created, initial planning
2. **Planned**: Requirements defined, team assigned
3. **InPreparation**: Resources allocated, tasks created
4. **InProgress**: Active development/work
5. **OnHold**: Temporarily paused
6. **Completed**: Work finished, delivered
7. **Cancelled**: Project terminated
8. **Deferred**: Postponed to future date

### State Transitions

```
Initiated → Planned → InPreparation → InProgress → Completed
                                           ↓
                                        OnHold
                                           ↓
                                      (Resume to InProgress)
                                           
Cancelled/Deferred (from any state)
```

## Project Structure

### Basic Project Information
```json
{
  "id": 1,
  "name": "E-commerce Website",
  "description": "Build a modern e-commerce platform",
  "budget": 10000,
  "client_id": "uuid",
  "freelancer_id": "uuid",
  "start_date": "2024-01-01",
  "end_date": "2024-06-30",
  "category": "Web Development",
  "status": "InProgress"
}
```

### Project Features

#### 1. Multi-Member Projects
Assign multiple freelancers to a single project:
```rust
project.members = vec![
    ProjectMember {
        user_id: "freelancer1_uuid",
        role: "Lead Developer"
    },
    ProjectMember {
        user_id: "freelancer2_uuid",
        role: "Designer"
    }
];
```

#### 2. Project Dependencies
Define dependencies between projects:
```rust
let dependency = Dependency {
    project_id: 2,
    depends_on: 1  // Project 2 depends on Project 1
};
```

#### 3. Project Templates
Create reusable project templates:
```rust
let template = ProjectTemplate {
    name: "Standard Website Template",
    default_budget: 5000,
    default_category: "Web Development",
    default_description: "Template for website projects"
};
```

## Task Management

### Task Structure
```json
{
  "id": 1,
  "project_id": 1,
  "task_list_id": 1,
  "title": "Design homepage mockup",
  "description": "Create responsive homepage design",
  "status": "in_progress",
  "assigned_to": "user_id",
  "due_date": "2024-02-15",
  "priority": 1
}
```

### Task States
- **Pending**: Not started
- **In Progress**: Currently being worked on
- **Completed**: Finished
- **Blocked**: Waiting on dependencies

### Task Lists
Group related tasks:
```rust
let task_list = TaskList {
    id: 1,
    project_id: 1,
    name: "Design Phase",
    tasks: vec![task1, task2, task3]
};
```

### Task Dependencies
```rust
let dependency = TaskDependency {
    task_id: 2,
    depends_on_task_id: 1  // Task 2 depends on Task 1
};
```

## Milestones

### Milestone Management

Create project milestones with deliverables:
```json
{
  "title": "Phase 1 Completion",
  "description": "Complete design and frontend development",
  "due_date": "2024-03-31",
  "payment_amount": 3000,
  "status": "in_progress"
}
```

### Milestone-Based Payments
- Payments tied to milestone completion
- Client approval required for payment release
- Escrow holds funds until milestone approved

### Milestone Workflow
1. Freelancer completes work
2. Freelancer marks milestone as complete
3. Client reviews deliverables
4. Client approves/rejects
5. If approved, payment released from escrow
6. If rejected, back to work or dispute

## Workflows

### Custom Workflows
Define project workflows:
```rust
let workflow = Workflow {
    id: 1,
    name: "Agile Development",
    stages: vec![
        "Planning",
        "Sprint 1",
        "Sprint 2",
        "Testing",
        "Deployment"
    ]
};
```

### Workflow Automation
- Automatic task creation
- Status transitions
- Notifications
- Deadline reminders

## Time Tracking

### Time Entry
```rust
let entry = TimeEntry {
    user_id: "freelancer_uuid",
    project_id: 1,
    task_id: Some(5),
    hours: 4.5,
    description: "Implemented user authentication",
    date: "2024-01-15"
};
```

### Timesheets
Generate timesheets for billing:
```rust
let timesheet = generate_timesheet(
    user_id,
    start_date,
    end_date
);
```

### Time Reports
```rust
let report = generate_time_report(
    project_id,
    start_date,
    end_date
);
// Returns total hours, billable hours, breakdown by task
```

## Collaboration Features

### Comments
Add comments to projects/tasks:
```rust
let comment = Comment {
    project_id: 1,
    user_id: "user_uuid",
    content: "Great progress on the design!",
    created_at: Utc::now()
};
```

### File Attachments
Attach files to projects/tasks:
```rust
let attachment = FileAttachment {
    project_id: 1,
    task_id: Some(5),
    file_url: "https://storage/file.pdf",
    uploaded_by: "user_uuid"
};
```

## Project Analytics

### Progress Tracking
- Completion percentage
- Time spent vs estimated
- Budget utilization
- Milestone status

### Reports
- Project status report
- Time tracking report
- Budget analysis
- Team performance

## API Endpoints

### Projects
```http
POST   /api/v1/projects              # Create project
GET    /api/v1/projects              # List all projects
GET    /api/v1/projects/{id}         # Get project
PUT    /api/v1/projects/{id}         # Update project
DELETE /api/v1/projects/{id}         # Delete project
```

### Tasks
```http
POST   /api/v1/tasks                 # Create task
GET    /api/v1/tasks/project/{id}    # Get project tasks
PUT    /api/v1/tasks/{id}            # Update task
DELETE /api/v1/tasks/{id}            # Delete task
```

### Milestones
```http
POST   /api/v1/milestones/{contract_id}              # Create milestone
GET    /api/v1/milestones/{contract_id}              # Get milestones
PUT    /api/v1/milestones/{contract_id}/{milestone_id}   # Update milestone
POST   /api/v1/milestones/{contract_id}/{milestone_id}/complete  # Mark complete
```

### Time Tracking
```http
POST   /api/v1/time-tracking/create         # Create time entry
GET    /api/v1/time-tracking/{user_id}      # Get entries
GET    /api/v1/time-tracking/{user_id}/timesheet  # Generate timesheet
```

## Best Practices

### Project Setup
1. Define clear objectives
2. Set realistic deadlines
3. Break down into tasks
4. Assign responsibilities
5. Establish milestones

### Task Management
1. Keep tasks small and focused
2. Set dependencies correctly
3. Update status regularly
4. Use priorities effectively
5. Add detailed descriptions

### Time Tracking
1. Log time daily
2. Be specific in descriptions
3. Categorize by task
4. Review timesheets weekly
5. Submit for approval promptly

### Collaboration
1. Comment frequently
2. Tag relevant users
3. Attach supporting files
4. Respond to questions
5. Keep everyone informed
