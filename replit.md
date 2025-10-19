# Workanda - The Future of Freelancing

## Overview
Workanda is a next-generation freelance marketplace that connects clients with skilled freelancers. The platform ensures secure payments through escrow, provides multi-currency support, Web3 integration, and AI-driven trust scores for a seamless freelancing experience.

## Project Structure

### Backend (Rust)
- **Location**: `/src` (root directory)
- **Framework**: Actix-web
- **Features**:
  - Authentication and authorization
  - Job posting and management
  - Proposal submission and review
  - Escrow payment system
  - Milestone tracking
  - User profiles (clients & freelancers)
  - Reviews and ratings
  - File uploads
  - Real-time chat
  - Admin dashboard

### Frontend (React + Vite)
- **Location**: `/frontend`
- **Framework**: React with Vite
- **Pages**:
  - **Home**: Landing page with features and how-it-works
  - **Browse Jobs**: Job listings with category filters
  - **Post a Job**: Form for clients to create new projects
  - **Dashboard**: Freelancer dashboard with active projects and proposals

## Recent Changes (Oct 19, 2025)

### Frontend Setup
- Created React + Vite frontend application
- Implemented responsive navigation with routing
- Built 4 main pages (Home, Jobs, Post Job, Dashboard)
- Added modern UI with gradient hero section
- Configured Vite to run on port 5000 with proxy to backend API
- Styled with custom CSS (purple/blue theme matching Workanda branding)

## Architecture

### Backend Stack
- **Language**: Rust
- **Web Framework**: Actix-web
- **Database**: PostgreSQL (via SQLx)
- **Authentication**: JWT tokens
- **Password Hashing**: Bcrypt
- **Event Streaming**: Kafka (optional)
- **Caching**: Redis

### Frontend Stack
- **Framework**: React 18
- **Build Tool**: Vite
- **Routing**: React Router v6
- **Styling**: Custom CSS with CSS variables
- **API Communication**: Fetch API (proxied through Vite)

## Configuration

### Environment Variables
The backend uses the following environment variables:
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET_KEY`: Secret key for JWT token signing
- `REDIS_URL`: Redis connection URL (optional)

### Development
- **Frontend Port**: 5000 (configured in vite.config.js)
- **Backend Port**: 8001 (configured in main.rs)
- **API Proxy**: Frontend proxies `/api/*` requests to `http://localhost:8001`

## Key Features

### For Clients
- Post jobs with detailed descriptions
- Set budgets and project timelines
- Review freelancer proposals
- Track project milestones
- Secure escrow payments
- Rate and review freelancers

### For Freelancers
- Browse available jobs by category
- Submit competitive proposals
- Manage multiple projects
- Track earnings and payments
- Build reputation through reviews
- Portfolio and skill management

### Platform Features
- 🔒 **Escrow Protection**: Payments held securely until work completion
- 💰 **Fair Fees**: Competitive platform fees
- 🌍 **Multi-Currency**: Support for fiat and crypto payments
- 🤖 **AI Trust Score**: Prevents scams and builds credibility
- ✅ **Milestone Tracking**: Integrated task management
- 📱 **Telegram Bot**: Instant job alerts and updates

## User Preferences
- Modern, clean UI design
- Purple/blue gradient color scheme
- Responsive design for all devices
- Fast page loads with Vite

## Future Enhancements
- Connect frontend forms to backend API endpoints
- Implement real authentication system
- Add payment integration (Stripe/crypto)
- Build real-time chat functionality
- Implement file upload system
- Add user profile pages
- Create search and filter functionality
- Implement notifications system
- Add analytics dashboard

## Running the Project

The frontend is configured to run automatically via the Replit workflow:
```bash
cd frontend && npm run dev
```

To run the backend separately (when needed):
```bash
cargo run --release
```

## Notes
- Frontend uses demo data for prototyping
- Backend API integration pending
- Database migrations in `src/database/migrations/`
- All routes are defined but need backend connection
