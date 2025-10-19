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
              <Link to="/post-job" className="nav-link">Post a Job</Link>
              <Link to="/dashboard" className="nav-link">Dashboard</Link>
              <Link to="/messages" className="nav-link">Messages</Link>
              <Link to="/profile" className="nav-link">Profile</Link>
              <ThemeToggle />
              <Link to="/login" className="btn btn-primary nav-btn">Login</Link>
            </div>
          </div>
        </nav>

        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/jobs" element={<Jobs />} />
          <Route path="/jobs/:id" element={<JobDetails />} />
          <Route path="/post-job" element={<PostJob />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/login" element={<Login />} />
          <Route path="/signup" element={<Signup />} />
          <Route path="/profile" element={<Profile />} />
          <Route path="/profile/:id" element={<Profile />} />
          <Route path="/messages" element={<Messages />} />
          <Route path="/proposals" element={<Proposals />} />
          <Route path="/projects" element={<Projects />} />
          <Route path="/projects/:id" element={<Projects />} />
          <Route path="/reviews" element={<Reviews />} />
          <Route path="/admin" element={<AdminDashboard />} />
        </Routes>

        <footer className="footer">
          <p>© 2025 Workanda - The Future of Freelancing</p>
        </footer>
      </div>
    </Router>
  )
}

export default App
