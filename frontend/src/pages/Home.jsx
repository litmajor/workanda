
import { Link } from 'react-router-dom'
import './Home.css'

function Home() {
  return (
    <div className="home">
      <section className="hero">
        <div className="hero-content">
          <div className="hero-badge">🚀 AI-Powered Freelance Marketplace</div>
          <h1 className="hero-title">
            The Future of Freelancing is Here
          </h1>
          <p className="hero-subtitle">
            Secure • Transparent • Intelligent • Empowering
          </p>
          <p className="hero-description">
            Connect with talented freelancers worldwide through our AI-powered matching engine. 
            Safe escrow payments, multi-currency support, predictive analytics, and comprehensive 
            trust & safety features designed for the modern gig economy.
          </p>
          <div className="hero-buttons">
            <Link to="/jobs" className="btn btn-primary">Browse Jobs</Link>
            <Link to="/post-job" className="btn btn-secondary">Post a Job</Link>
          </div>
          <div className="hero-stats">
            <div className="stat">
              <strong>15,000+</strong>
              <span>Active Users</span>
            </div>
            <div className="stat">
              <strong>2,300+</strong>
              <span>Projects Completed</span>
            </div>
            <div className="stat">
              <strong>98%</strong>
              <span>Success Rate</span>
            </div>
          </div>
        </div>
      </section>

      <section className="features">
        <h2>Core Platform Features</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">🔒</div>
            <h3>Secure Escrow Payments</h3>
            <p>Funds held safely until work is completed and approved</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💰</div>
            <h3>Multi-Currency Support</h3>
            <p>Accept payments in fiat and cryptocurrencies</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">✅</div>
            <h3>Milestone Tracking</h3>
            <p>Break projects into manageable milestones with automated payments</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📊</div>
            <h3>Project Management</h3>
            <p>Built-in task management, time tracking, and progress monitoring</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💬</div>
            <h3>Real-time Messaging</h3>
            <p>Instant communication with clients and team members</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📱</div>
            <h3>Telegram Integration</h3>
            <p>Get instant notifications for jobs and messages</p>
          </div>
        </div>
      </section>

      <section className="ai-features">
        <h2>🤖 AI-Powered Intelligence</h2>
        <p className="section-subtitle">Revolutionizing freelancing with artificial intelligence</p>
        <div className="features-grid">
          <div className="feature-card highlight">
            <div className="feature-icon">🎯</div>
            <h3>Smart Matching</h3>
            <p>AI analyzes skills, experience, and success rates to match freelancers with ideal projects</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">👥</div>
            <h3>Team Formation</h3>
            <p>Automatically suggest optimal team compositions based on skill synergy and compatibility</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">📈</div>
            <h3>Success Prediction</h3>
            <p>Predict project success probability before starting to minimize risks</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">💵</div>
            <h3>Dynamic Pricing</h3>
            <p>AI-suggested pricing based on market trends and project complexity</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">⏱️</div>
            <h3>Timeline Estimation</h3>
            <p>Accurate delivery predictions using historical data and team velocity</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">✍️</div>
            <h3>Proposal Assistant</h3>
            <p>AI helps craft winning proposals with optimization suggestions</p>
          </div>
        </div>
      </section>

      <section className="trust-safety">
        <h2>🛡️ Trust & Safety</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">⭐</div>
            <h3>Multi-Dimensional Trust Scores</h3>
            <p>Beyond reviews: reliability, communication, quality, and professionalism metrics</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🚨</div>
            <h3>Fraud Detection</h3>
            <p>AI-powered identification of suspicious activity and fake profiles</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">⚖️</div>
            <h3>Dispute Prevention</h3>
            <p>Proactive alerts and mediation recommendations to avoid conflicts</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🔍</div>
            <h3>Behavioral Analysis</h3>
            <p>Track patterns to ensure platform integrity and user safety</p>
          </div>
        </div>
      </section>

      <section className="collaboration">
        <h2>🤝 Team Collaboration</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">👨‍👩‍👧‍👦</div>
            <h3>Team Formation</h3>
            <p>Create and manage freelance teams for complex projects</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🏢</div>
            <h3>Agency Support</h3>
            <p>Run your freelance agency with team proposals and revenue distribution</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💸</div>
            <h3>Smart Revenue Split</h3>
            <p>Automated payment distribution based on contribution and roles</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📋</div>
            <h3>Unified Proposals</h3>
            <p>Submit team proposals with clear role definitions and pricing</p>
          </div>
        </div>
      </section>

      <section className="how-it-works">
        <h2>How It Works</h2>
        <div className="steps">
          <div className="step">
            <div className="step-number">1</div>
            <h3>Post a Job</h3>
            <p>Clients describe work, AI categorizes and suggests pricing</p>
          </div>
          <div className="step">
            <div className="step-number">2</div>
            <h3>AI Matching</h3>
            <p>Smart algorithm matches best freelancers or teams</p>
          </div>
          <div className="step">
            <div className="step-number">3</div>
            <h3>Submit Proposals</h3>
            <p>Freelancers bid with AI-assisted proposal writing</p>
          </div>
          <div className="step">
            <div className="step-number">4</div>
            <h3>Secure Contract</h3>
            <p>Escrow protection with milestone-based payments</p>
          </div>
          <div className="step">
            <div className="step-number">5</div>
            <h3>Track Progress</h3>
            <p>Monitor tasks, time, and milestones in real-time</p>
          </div>
          <div className="step">
            <div className="step-number">6</div>
            <h3>Complete & Review</h3>
            <p>Automated payment release and mutual feedback</p>
          </div>
        </div>
      </section>

      <section className="cta-section">
        <h2>Ready to Transform Your Freelance Journey?</h2>
        <p>Join thousands of successful freelancers and clients on Workanda</p>
        <div className="cta-buttons">
          <Link to="/signup" className="btn btn-primary btn-large">Get Started Free</Link>
          <Link to="/jobs" className="btn btn-secondary btn-large">Explore Opportunities</Link>
        </div>
      </section>
    </div>
  )
}

export default Home
