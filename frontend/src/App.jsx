import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import Home from './pages/Home'
import Jobs from './pages/Jobs'
import PostJob from './pages/PostJob'
import Dashboard from './pages/Dashboard'
import Login from './pages/Login'
import Signup from './pages/Signup'
import Profile from './pages/Profile'
import JobDetails from './pages/JobDetails'
import Messages from './pages/Messages'
import Proposals from './pages/Proposals'
import Projects from './pages/Projects'
import Reviews from './pages/Reviews'
import AdminDashboard from './pages/AdminDashboard'
import Progress from './pages/Progress'
import Settings from './pages/Settings'
import Notifications from './pages/Notifications'
import Teams from './pages/Teams'
import Analytics from './pages/Analytics'
import ThemeToggle from './components/ThemeToggle'
import './App.css'

function App() {
  return (
    <Router>
      <div className="app">
        <nav className="navbar">
          <div className="nav-container">
            <Link to="/" className="logo">
              <span className="logo-icon">🚀</span>
              Workanda
            </Link>
            <div className="nav-links">
              <Link to="/jobs" className="nav-link">Browse Jobs</Link>
              <Link to="/post-job" className="nav-link">Post Job</Link>
              <Link to="/dashboard" className="nav-link">Dashboard</Link>
              <Link to="/analytics" className="nav-link">Analytics</Link>
              <Link to="/messages" className="nav-link">Messages</Link>
              <Link to="/profile" className="nav-link">Profile</Link>
              <Link to="/progress" className="nav-link">Progress</Link>
              <ThemeToggle />
              <Link to="/login" className="btn btn-primary nav-btn">Login</Link>
            </div>
          </div>
        </nav>

        <Routes>
          {/* Public Routes */}
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/signup" element={<Signup />} />
          
          {/* Job Routes */}
          <Route path="/jobs" element={<Jobs />} />
          <Route path="/jobs/:id" element={<JobDetails />} />
          <Route path="/post-job" element={<PostJob />} />
          
          {/* User Routes */}
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/profile" element={<Profile />} />
          <Route path="/profile/:id" element={<Profile />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/notifications" element={<Notifications />} />
          
          {/* Project & Proposal Routes */}
          <Route path="/proposals" element={<Proposals />} />
          <Route path="/proposals/:id" element={<Proposals />} />
          <Route path="/projects" element={<Projects />} />
          <Route path="/projects/:id" element={<Projects />} />
          
          {/* Communication Routes */}
          <Route path="/messages" element={<Messages />} />
          <Route path="/messages/:userId" element={<Messages />} />
          
          {/* Team & Collaboration Routes */}
          <Route path="/teams" element={<Teams />} />
          <Route path="/teams/:id" element={<Teams />} />
          
          {/* Review & Rating Routes */}
          <Route path="/reviews" element={<Reviews />} />
          <Route path="/reviews/:userId" element={<Reviews />} />
          
          {/* Analytics & Reports */}
          <Route path="/analytics" element={<Analytics />} />
          
          {/* Admin Routes */}
          <Route path="/admin" element={<AdminDashboard />} />
          <Route path="/admin/users" element={<AdminDashboard />} />
          <Route path="/admin/jobs" element={<AdminDashboard />} />
          <Route path="/admin/disputes" element={<AdminDashboard />} />
          <Route path="/admin/analytics" element={<AdminDashboard />} />
          
          {/* Development/Progress Route */}
          <Route path="/progress" element={<Progress />} />
        </Routes>

        <footer className="footer">
          <p>© 2025 Workanda - The Future of Freelancing</p>
        </footer>
      </div>
    </Router>
  )
}

export default App
