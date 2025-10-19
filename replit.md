# Workanda - The Future of Freelancing

## Overview
Workanda is a next-generation AI-powered freelance marketplace designed to connect African talent with global opportunities. The platform ensures secure payments through escrow, provides multi-currency support, AI-driven matching, and comprehensive trust & safety features for a seamless freelancing experience.

## Vision & Mission
**African Stronghold**: Workanda is built to empower African freelancers by providing niche specialization capabilities, local payment methods, and regional focus while connecting them to global clients.

**Niche Specialization**: Freelancers can define their exact expertise (e.g., Legal Transcriptionist, Law Enforcement Transcriptionist, API Documentation Writer) to stand out and attract the right clients.

## Project Structure

### Backend (Rust - Actix-web)
- **Location**: `/src` (root directory)
- **Framework**: Actix-web
- **Database**: PostgreSQL (via SQLx)
- **Authentication**: JWT tokens, OAuth2 (Google), 2FA/TOTP
- **Caching**: Redis
- **Event Streaming**: Kafka (optional)

### Frontend (React + Vite)
- **Location**: `/frontend`
- **Framework**: React 19.1 with Vite 7.1
- **Routing**: React Router v7
- **Styling**: Custom CSS with CSS variables, responsive design
- **API Communication**: Centralized API service layer

## Recent Changes (Oct 19, 2025)

### Full Frontend Implementation
**Completed Today:**
- ✅ Fixed initial import issues (npm dependencies, syntax errors)
- ✅ Created comprehensive API service layer (`services/api.js`)
- ✅ Built category and specialization system with African focus
- ✅ Implemented AI Matching page with compatibility scores
- ✅ Built AI Proposal Assistant with win rate predictions
- ✅ Created Freelancer Profile Setup with niche specialization flow
- ✅ Implemented Escrow Management with milestone-based payments
- ✅ Built Time Tracking interface for freelancers
- ✅ Updated navigation and routing for all new features

## Core Features

### 🤖 AI-Powered Features

#### 1. AI Matching (`/ai/matching`)
- Smart freelancer-to-project matching
- Compatibility scores based on:
  - Skills and proficiency levels
  - Experience and track record
  - Budget alignment
  - Success probability
- Detailed match explanations
- Team composition suggestions

#### 2. Proposal Assistant (`/ai/proposal-assistant`)
- AI-powered proposal writing assistance
- Win rate prediction (estimated probability of success)
- Quality scoring (structure, relevance, clarity, professionalism)
- Personalized suggestions based on job requirements
- Client preference analysis
- Key points and recommended structure

#### 3. Predictive Analytics
- Project success prediction
- Dynamic pricing suggestions
- Timeline estimation
- Risk assessment

#### 4. Trust & Safety
- AI-powered trust scores
- Fraud detection
- Dispute risk prediction
- Behavioral analytics

### 👤 User Management

#### For Freelancers
**Profile Setup** (`/freelancer/setup`)
- Category selection (Writing, Design, Development, Data, Marketing, Business, Admin, Engineering, Audio)
- Specialization selection within categories
- **Niche definition** (e.g., Legal Transcriptionist, Medical Transcriptionist, Law Enforcement)
- Hourly and project-based pricing
- Location and availability status
- Skills with proficiency levels
- Portfolio showcase
- Trust score display

**Features:**
- Browse AI-matched jobs
- Submit proposals with AI assistance
- Manage multiple projects
- Track time and earnings
- Build reputation through reviews
- Team collaboration

#### For Clients
**Profile Features:**
- Company information
- Industry/sector specification
- Hiring preferences
- Trust score

**Features:**
- Post detailed job listings
- Review AI-matched freelancers
- Manage proposals
- Milestone-based project tracking
- Secure escrow payments
- Rate and review freelancers

### 💰 Escrow & Payment System (`/escrow`)
- Secure escrow account management
- Milestone-based payment releases
- Fund hold until work completion
- Dispute resolution system
- Payment history tracking
- Automatic fee calculation
- Multi-currency support
- African payment methods (M-Pesa, Flutterwave, Paystack)

### 📊 Project & Milestone Management
- Full project lifecycle management
- Task and subtask creation
- Milestone tracking
- Workflow stages
- Dependency management
- Progress monitoring
- Time tracking integration
- File uploads and sharing

### ⏱️ Time Tracking (`/time-tracking`)
- Active timer for work sessions
- Time entry logging
- Billable vs non-billable hours
- Timesheet management
- Earnings calculator
- Weekly/monthly reports

### 💬 Communication
- Real-time messaging system
- Chat rooms for projects
- Notification system
- File sharing in messages

### 👥 Teams & Agencies
- Team creation and management
- Agency dashboards
- Team proposals
- Revenue distribution
- Collaborative workspaces

### ⭐ Reviews & Ratings
- Add/edit reviews
- Paginated review display
- Aggregate ratings
- Review moderation

### 🛡️ Admin Features
- User management
- Job approval and moderation
- Dispute resolution
- System analytics
- Membership tier management
- Activity logs

## African-Focused Features

### Payment Methods
- M-Pesa (East Africa)
- Flutterwave (West & East Africa)
- Paystack (West Africa)
- Bank Transfer
- Cryptocurrency
- Stripe

### Currency Support
- USD, EUR, GBP
- NGN (Nigerian Naira)
- KES (Kenyan Shilling)
- ZAR (South African Rand)
- GHS (Ghanaian Cedi)
- EGP (Egyptian Pound)

### Regional Categories
Freelancers can specify their region:
- West Africa (Nigeria, Ghana, Senegal, Ivory Coast, Mali)
- East Africa (Kenya, Tanzania, Uganda, Ethiopia, Rwanda)
- Southern Africa (South Africa, Botswana, Namibia, Zimbabwe, Zambia)
- North Africa (Egypt, Morocco, Tunisia, Algeria, Libya)
- Central Africa (Cameroon, DRC, Gabon, Chad, CAR)

## Technical Architecture

### API Structure
**Base URL**: `/api/v1`

**Endpoints:**
- `/auth/*` - Authentication (login, register, logout, OAuth)
- `/freelancer/*` - Freelancer profile and job browsing
- `/client/*` - Client profile and job posting
- `/jobs/*` - Job listings and details
- `/proposals/*` - Proposal management
- `/projects/*` - Project CRUD and team management
- `/contracts/*` - Contract creation and management
- `/milestones/*` - Milestone tracking
- `/escrow/*` - Escrow and payment management
- `/payments/*` - Payment processing and history
- `/reviews/*` - Review system
- `/messages/*` - Messaging system
- `/teams/*` - Team management
- `/agencies/*` - Agency management
- `/ai/matches/*` - AI matching service
- `/ai/proposal/*` - Proposal assistant
- `/ai/team/*` - Team composition AI
- `/predictive/*` - Predictive analytics
- `/trust/*` - Trust & safety features

### Data Models

**User Management:**
- User (with role-based access)
- FreelancerAccount (with specializations, niches, rates)
- ClientAccount (with company details)

**Work Management:**
- Job (with category, skills, budget)
- Proposal (with bid amount, message)
- Project (with milestones, tasks, dependencies)
- Contract (with terms, parties)
- Milestone (with payment amount, due date)

**Financial:**
- Escrow (with holds and releases)
- Payment (with history)
- Fee (fixed and percentage-based)

**Communication:**
- Message
- Notification
- ChatRoom

**Teams:**
- Team
- Agency
- TeamProposal
- RevenueDistribution

**Quality:**
- Review
- Rating
- TrustScore

## Configuration

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET_KEY`: Secret key for JWT token signing
- `REDIS_URL`: Redis connection URL
- `GOOGLE_CLIENT_ID`: OAuth2 Google client ID
- `GOOGLE_CLIENT_SECRET`: OAuth2 Google client secret

### Development Ports
- **Frontend**: 5000 (Vite dev server)
- **Backend**: 8001 (Actix-web server)
- **API Proxy**: Frontend proxies `/api/*` to `http://localhost:8001`

## Running the Project

### Frontend (Auto-starts via Replit workflow)
```bash
cd frontend && npm run dev
```

### Backend (Manual start when needed)
```bash
cargo run --release
```

## Pages & Routes

### Public
- `/` - Home/Landing page
- `/login` - User login
- `/signup` - User registration
- `/jobs` - Browse all jobs
- `/jobs/:id` - Job details

### Freelancer
- `/dashboard` - Freelancer dashboard
- `/freelancer/setup` - Complete profile with niche specialization
- `/ai/matching` - AI-matched job recommendations
- `/ai/proposal-assistant` - AI-powered proposal help
- `/time-tracking` - Track billable hours
- `/proposals` - Manage proposals
- `/projects` - Active projects

### Client
- `/post-job` - Create new job posting
- `/proposals/:jobId` - Review proposals for a job
- `/escrow` - Manage payments and milestones
- `/projects` - Manage active projects

### Shared
- `/profile` - User profile (view/edit)
- `/messages` - Communication center
- `/teams` - Team management
- `/reviews` - Reviews and ratings
- `/settings` - Account settings
- `/notifications` - Notifications
- `/analytics` - Performance analytics

### Admin
- `/admin` - Admin dashboard
- `/admin/users` - User management
- `/admin/jobs` - Job moderation
- `/admin/disputes` - Dispute resolution
- `/admin/analytics` - System analytics

## Categories & Specializations

### Main Categories (9)
1. **Writing & Translation** - Content, Copywriting, Technical, Creative, Transcription (Legal, Medical, Law Enforcement), Translation, Editing
2. **Design & Creative** - Graphic Design, UI/UX, Illustration, Video/Animation, Photography
3. **Development & IT** - Web, Mobile, Software, Game Dev, Blockchain, DevOps, Cybersecurity
4. **Data & Analytics** - Data Science, Data Analysis, Data Engineering
5. **Marketing & Sales** - Digital Marketing, Content Marketing, Sales, Market Research
6. **Business & Consulting** - Business, Financial, Legal, HR
7. **Admin & Customer Support** - Virtual Assistant, Customer Support, Project Management
8. **Engineering & Architecture** - CAD, Architecture, Mechanical Engineering
9. **Audio & Music** - Music Production, Voice Over, Audio Editing

**Total Specializations**: 40+
**Total Niches**: 150+

## User Preferences
- Modern, clean UI design
- Purple/blue gradient color scheme (#6366f1, #8b5cf6)
- Responsive design for all devices
- Fast page loads with Vite
- Accessibility features
- Dark/light theme toggle
- African market focus

## Next Steps

### Backend Integration (Priority)
- [ ] Connect all frontend pages to backend API
- [ ] Implement authentication flow
- [ ] Add error handling and loading states
- [ ] Test all AI features with real backend

### Features to Enhance
- [ ] Real-time notifications
- [ ] WebSocket for live chat
- [ ] Advanced search and filters
- [ ] File upload system
- [ ] Payment gateway integration (Stripe, M-Pesa, Flutterwave)
- [ ] Email notification system
- [ ] Mobile app (React Native)

### African Market Features
- [ ] Local language support (Swahili, Yoruba, Zulu, etc.)
- [ ] Regional job categories
- [ ] Currency conversion
- [ ] Timezone handling
- [ ] Mobile money integration
- [ ] SMS notifications for low-internet areas

## Development Notes

- Frontend runs on port 5000 (only port accessible in Replit)
- Backend uses SQLx with compile-time query verification
- All routes are protected with JWT authentication
- RBAC (Role-Based Access Control) implemented
- Rate limiting on sensitive endpoints
- Database migrations in `src/database/migrations/`
- Passwords hashed with Bcrypt
- 2FA/TOTP support available
- OAuth2 (Google) integration ready

## Platform Benefits

### For Freelancers
- 🎯 Niche specialization for better targeting
- 🤖 AI-powered job matching
- 💰 Transparent pricing with escrow protection
- 📈 Career growth through analytics
- 🌍 Global opportunities, African-first
- ⏱️ Easy time tracking and invoicing
- 👥 Team collaboration features

### For Clients
- 🔍 Find specialized talent quickly
- 🤖 AI-recommended freelancers
- 🔒 Secure milestone-based payments
- 📊 Project progress tracking
- ⭐ Verified reviews and trust scores
- 🌍 Access to African talent pool
- 💬 Direct communication tools

### Platform Advantages
- Lower fees than competitors
- AI-driven matching and recommendations
- Strong trust & safety features
- African payment methods
- Multi-currency support
- Comprehensive dispute resolution
- Focus on quality over quantity

---

**Built with ❤️ for the African freelance community**

_Last Updated: October 19, 2025_
