
# Frontend Routes Documentation

## Public Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/` | Home | Landing page with feature showcase | No |
| `/login` | Login | User login page | No |
| `/signup` | Signup | User registration page | No |

## Job Management Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/jobs` | Jobs | Browse all available jobs with filters | No |
| `/jobs/:id` | JobDetails | View detailed job information | No |
| `/post-job` | PostJob | Create a new job posting | Yes (Client) |

## User Profile Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/dashboard` | Dashboard | User dashboard with stats and overview | Yes |
| `/profile` | Profile | View/edit own profile | Yes |
| `/profile/:id` | Profile | View other user's profile | No |
| `/settings` | Settings | User account settings | Yes |
| `/notifications` | Notifications | Notification center | Yes |

## Project & Proposal Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/proposals` | Proposals | View all proposals | Yes |
| `/proposals/:id` | Proposals | View specific proposal details | Yes |
| `/projects` | Projects | View all projects | Yes |
| `/projects/:id` | Projects | View specific project details | Yes |

## Communication Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/messages` | Messages | Message inbox | Yes |
| `/messages/:userId` | Messages | Direct message with specific user | Yes |

## Team & Collaboration Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/teams` | Teams | View all teams | Yes |
| `/teams/:id` | Teams | View specific team details | Yes |

## Review & Rating Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/reviews` | Reviews | View all reviews | Yes |
| `/reviews/:userId` | Reviews | View reviews for specific user | No |

## Analytics Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/analytics` | Analytics | Analytics dashboard with charts | Yes |

## Admin Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/admin` | AdminDashboard | Admin dashboard overview | Yes (Admin) |
| `/admin/users` | AdminDashboard | User management | Yes (Admin) |
| `/admin/jobs` | AdminDashboard | Job moderation | Yes (Admin) |
| `/admin/disputes` | AdminDashboard | Dispute resolution | Yes (Admin) |
| `/admin/analytics` | AdminDashboard | Platform analytics | Yes (Admin) |

## Development Routes

| Route | Component | Description | Auth Required |
|-------|-----------|-------------|---------------|
| `/progress` | Progress | Project progress tracker | No |

## Route Parameters

### Dynamic Parameters

- `:id` - Numeric or UUID identifier for resources (jobs, projects, proposals, teams)
- `:userId` - User identifier for profiles and messages

### Query Parameters (Planned)

- `?page=` - Pagination
- `?sort=` - Sorting options
- `?filter=` - Filtering criteria
- `?search=` - Search terms

## Protected Routes

All routes marked with "Yes" in Auth Required column should be protected by authentication middleware. Implement route guards to:

1. Check if user is authenticated
2. Verify user role/permissions
3. Redirect to `/login` if not authenticated
4. Show 403 error if insufficient permissions

## Future Routes (Planned)

- `/contracts` - Contract management
- `/payments` - Payment history
- `/invoices` - Invoice management
- `/disputes` - Dispute center
- `/escrow` - Escrow management
- `/time-tracking` - Time tracking interface
- `/tasks` - Task management
- `/calendar` - Calendar view
- `/files` - File manager
- `/search` - Advanced search
