import { Link } from 'react-router-dom'
import './Home.css'

function Home() {
  return (
    <div className="home">
      <section className="hero">
        <div className="hero-content">
          <h1 className="hero-title">
            The Future of Freelancing
          </h1>
          <p className="hero-subtitle">
            Secure, Transparent, and Empowering
          </p>
          <p className="hero-description">
            Connect with talented freelancers worldwide. Safe payments through escrow, 
            multi-currency support, and AI-driven trust scores.
          </p>
          <div className="hero-buttons">
            <Link to="/jobs" className="btn btn-primary">Browse Jobs</Link>
            <Link to="/post-job" className="btn btn-secondary">Post a Job</Link>
          </div>
        </div>
      </section>

      <section className="features">
        <h2>Why Choose Workanda?</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">🔒</div>
            <h3>Safe & Secure</h3>
            <p>Payments held in escrow until work is completed</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💰</div>
            <h3>Fair Fees</h3>
            <p>Low costs for freelancers, competitive rates</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🌍</div>
            <h3>Multi-Currency</h3>
            <p>Get paid in fiat or crypto currencies</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🤖</div>
            <h3>AI Trust Score</h3>
            <p>Prevents scams and builds credibility</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">✅</div>
            <h3>Milestone Tracking</h3>
            <p>Integrated task management system</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📱</div>
            <h3>Instant Alerts</h3>
            <p>Telegram bot for job notifications</p>
          </div>
        </div>
      </section>

      <section className="how-it-works">
        <h2>How It Works</h2>
        <div className="steps">
          <div className="step">
            <div className="step-number">1</div>
            <h3>Post a Job</h3>
            <p>Clients describe work and set budget</p>
          </div>
          <div className="step">
            <div className="step-number">2</div>
            <h3>Get Proposals</h3>
            <p>Freelancers submit bids</p>
          </div>
          <div className="step">
            <div className="step-number">3</div>
            <h3>Escrow Protection</h3>
            <p>Payment held securely</p>
          </div>
          <div className="step">
            <div className="step-number">4</div>
            <h3>Track Progress</h3>
            <p>Monitor milestones</p>
          </div>
          <div className="step">
            <div className="step-number">5</div>
            <h3>Release Payment</h3>
            <p>Approve and pay freelancer</p>
          </div>
        </div>
      </section>
    </div>
  )
}

export default Home
