import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import { useApp } from './context/AppContext'
import ThemeToggle from './components/ThemeToggle'
import ErrorBoundary from './components/ErrorBoundary'
import Alert from './components/Alert'
import { useWebSocket } from './hooks/useWebSocket'
import Home from './pages/Home'
import Jobs from './pages/Jobs'
import PostJob from './pages/PostJob'
import Dashboard from './pages/Dashboard'
import Login from './pages/Login'
import Signup from './pages/Signup'
import ForgotPassword from './pages/ForgotPassword'
import ResetPassword from './pages/ResetPassword'
import Profile from './pages/Profile'
import JobDetails from './pages/JobDetails'
import Messages from './pages/Messages'
import Proposals from './pages/Proposals'
import Projects from './pages/Projects'
import Reviews from './pages/Reviews'
import AdminDashboard from './pages/AdminDashboard'
import Progress from './pages/Progress';
import Wallet from './pages/Wallet';
import BlockchainWallet from './pages/BlockchainWallet';
import { AppProvider } from './context/AppContext';
import Settings from './pages/Settings'
import Notifications from './pages/Notifications'
import Teams from './pages/Teams'
import Analytics from './pages/Analytics'
import TrustSafety from './pages/TrustSafety'
import AIMatching from './pages/AIMatching'
import ProposalAssistant from './pages/ProposalAssistant'
import FreelancerProfileSetup from './pages/FreelancerProfileSetup'
import EscrowManagement from './pages/EscrowManagement';
import Invoicing from './pages/Invoicing';
import './App.css'

function App() {
  const { user, alerts } = useApp()
  useWebSocket() // Initialize WebSocket connection

  return (
    <ErrorBoundary>
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
                <Link to="/ai/matching" className="nav-link">AI Matching</Link>
                <Link to="/dashboard" className="nav-link">Dashboard</Link>
                <Link to="/escrow" className="nav-link">Payments</Link>
                <Link to="/wallet" className="nav-link">Wallet</Link>
                <Link to="/wallet/blockchain" className="nav-link">🔐 Crypto</Link>
                <Link to="/trust-safety" className="nav-link">Trust & Safety</Link>
                <Link to="/profile" className="nav-link">Profile</Link>
                <ThemeToggle />
                <Link to="/login" className="btn btn-primary nav-btn">Login</Link>
              </div>
            </div>
          </nav>

          {alerts.length > 0 && <Alert alerts={alerts} />}

          <Routes>
            {/* Public Routes */}
            <Route path="/" element={<Home />} />
            <Route path="/login" element={<Login />} />
            <Route path="/signup" element={<Signup />} />
            <Route path="/forgot-password" element={<ForgotPassword />} />
            <Route path="/reset-password" element={<ResetPassword />} />

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

            {/* Trust & Safety */}
            <Route path="/trust-safety" element={<TrustSafety />} />

            {/* Review & Rating Routes */}
            <Route path="/reviews" element={<Reviews />} />
            <Route path="/reviews/:userId" element={<Reviews />} />

            {/* Analytics & Reports */}
            <Route path="/analytics" element={<Analytics />} />

            {/* AI Features */}
            <Route path="/ai/matching" element={<AIMatching />} />
            <Route path="/ai/proposal-assistant" element={<ProposalAssistant />} />

            {/* Freelancer Setup */}
            <Route path="/freelancer/setup" element={<FreelancerProfileSetup />} />

            {/* Escrow & Payments */}
            <Route path="/escrow" element={<EscrowManagement />} />
            <Route path="/invoicing" element={<Invoicing />} />
            <Route path="/ai-matching" element={<AIMatching />} />
            <Route path="/proposal-assistant" element={<ProposalAssistant />} />
            <Route path="/wallet" element={<Wallet />} />
            <Route path="/wallet/blockchain" element={<BlockchainWallet />} />

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
    </ErrorBoundary>
  )
}

export default App