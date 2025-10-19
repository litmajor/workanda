import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import Home from './pages/Home'
import Jobs from './pages/Jobs'
import PostJob from './pages/PostJob'
import Dashboard from './pages/Dashboard'
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
            </div>
          </div>
        </nav>

        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/jobs" element={<Jobs />} />
          <Route path="/post-job" element={<PostJob />} />
          <Route path="/dashboard" element={<Dashboard />} />
        </Routes>

        <footer className="footer">
          <p>© 2025 Workanda - The Future of Freelancing</p>
        </footer>
      </div>
    </Router>
  )
}

export default App
