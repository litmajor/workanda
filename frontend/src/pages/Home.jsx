
import { Link } from 'react-router-dom'
import './Home.css'

function Home() {
  return (
    <div className="home">
      <section className="hero">
        <div className="hero-content">
          <div className="hero-badge">🚀 Next-Generation Freelance Platform</div>
          <h1 className="hero-title">
            Where Talent Meets Opportunity
          </h1>
          <p className="hero-subtitle">
            Smart • Secure • Seamless • Scalable
          </p>
          <p className="hero-description">
            Workanda revolutionizes the freelance economy with AI-powered matching, 
            blockchain-secured payments, and intelligent project management. 
            Connect with top talent globally, manage complex projects effortlessly, 
            and build lasting professional relationships—all in one platform.
          </p>
          <div className="hero-buttons">
            <Link to="/jobs" className="btn btn-primary">Explore Opportunities</Link>
            <Link to="/post-job" className="btn btn-secondary">Hire Talent</Link>
          </div>
          <div className="hero-stats">
            <div className="stat">
              <strong>15,000+</strong>
              <span>Global Professionals</span>
            </div>
            <div className="stat">
              <strong>2,300+</strong>
              <span>Successful Projects</span>
            </div>
            <div className="stat">
              <strong>98%</strong>
              <span>Client Satisfaction</span>
            </div>
          </div>
        </div>
      </section>

      <section className="features">
        <h2>Platform Capabilities</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">🔒</div>
            <h3>Blockchain Escrow</h3>
            <p>Cryptocurrency and fiat payments held securely until milestone completion</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💎</div>
            <h3>Multi-Currency Payments</h3>
            <p>Accept Bitcoin, Ethereum, stablecoins, and traditional currencies</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🎯</div>
            <h3>Smart Milestones</h3>
            <p>Automated payment releases tied to verified deliverables</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📊</div>
            <h3>Advanced Analytics</h3>
            <p>Real-time insights into project health, team performance, and revenue</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💬</div>
            <h3>Integrated Communication</h3>
            <p>Built-in messaging, video calls, and Telegram notifications</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">⏱️</div>
            <h3>Time Tracking</h3>
            <p>Automatic and manual time tracking with detailed reporting</p>
          </div>
        </div>
      </section>

      <section className="ai-features">
        <h2>🤖 AI-Powered Intelligence</h2>
        <p className="section-subtitle">Leverage cutting-edge AI to optimize every aspect of freelancing</p>
        <div className="features-grid">
          <div className="feature-card highlight">
            <div className="feature-icon">🎯</div>
            <h3>Intelligent Matching</h3>
            <p>Machine learning algorithms match freelancers with projects based on skills, experience, success rates, and availability</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">👥</div>
            <h3>Team Optimization</h3>
            <p>AI suggests optimal team compositions analyzing skill complementarity and past collaboration success</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">📈</div>
            <h3>Predictive Analytics</h3>
            <p>Forecast project success probability, timeline accuracy, and potential risks before commitment</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">💵</div>
            <h3>Market-Based Pricing</h3>
            <p>Dynamic pricing recommendations based on real-time market data and project complexity</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">⏱️</div>
            <h3>Smart Scheduling</h3>
            <p>Accurate delivery predictions using historical data, team velocity, and project scope</p>
          </div>
          <div className="feature-card highlight">
            <div className="feature-icon">✍️</div>
            <h3>Proposal Assistant</h3>
            <p>AI-powered writing assistance to craft compelling, winning proposals</p>
          </div>
        </div>
      </section>

      <section className="trust-safety">
        <h2>🛡️ Trust & Safety Ecosystem</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">⭐</div>
            <h3>Comprehensive Trust Scores</h3>
            <p>Multi-dimensional ratings covering reliability, communication quality, technical skills, and professionalism</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🚨</div>
            <h3>AI Fraud Detection</h3>
            <p>Real-time behavioral analysis identifies suspicious activity and fraudulent profiles</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">⚖️</div>
            <h3>Dispute Prevention</h3>
            <p>Proactive alerts and mediation recommendations to resolve conflicts before escalation</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🔍</div>
            <h3>Behavioral Monitoring</h3>
            <p>Continuous pattern analysis ensures platform integrity and user safety</p>
          </div>
        </div>
      </section>

      <section className="collaboration">
        <h2>🤝 Collaboration & Teams</h2>
        <div className="features-grid">
          <div className="feature-card">
            <div className="feature-icon">👨‍👩‍👧‍👦</div>
            <h3>Dynamic Team Formation</h3>
            <p>Build specialized teams for complex, multi-disciplinary projects</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">🏢</div>
            <h3>Agency Management</h3>
            <p>Run your freelance agency with centralized billing, team proposals, and client management</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">💸</div>
            <h3>Automated Revenue Distribution</h3>
            <p>Fair payment splits based on contributions, roles, and predefined agreements</p>
          </div>
          <div className="feature-card">
            <div className="feature-icon">📋</div>
            <h3>Unified Team Proposals</h3>
            <p>Submit collaborative bids with clear role definitions and transparent pricing</p>
          </div>
        </div>
      </section>

      <section className="how-it-works">
        <h2>How Workanda Works</h2>
        <div className="steps">
          <div className="step">
            <div className="step-number">1</div>
            <h3>Post Your Project</h3>
            <p>Describe your needs, AI categorizes and suggests optimal pricing and timeline</p>
          </div>
          <div className="step">
            <div className="step-number">2</div>
            <h3>Receive AI Matches</h3>
            <p>Our algorithm identifies the best-fit freelancers or teams for your project</p>
          </div>
          <div className="step">
            <div className="step-number">3</div>
            <h3>Review Proposals</h3>
            <p>Compare bids, portfolios, trust scores, and AI success predictions</p>
          </div>
          <div className="step">
            <div className="step-number">4</div>
            <h3>Secure Agreement</h3>
            <p>Sign contracts with escrow-backed milestone payments</p>
          </div>
          <div className="step">
            <div className="step-number">5</div>
            <h3>Collaborate & Track</h3>
            <p>Monitor progress, communicate seamlessly, and approve milestones</p>
          </div>
          <div className="step">
            <div className="step-number">6</div>
            <h3>Complete & Review</h3>
            <p>Automated payment release and mutual feedback exchange</p>
          </div>
        </div>
      </section>

      <section className="cta-section">
        <h2>Ready to Transform Your Freelance Experience?</h2>
        <p>Join the next generation of freelancers and businesses on Workanda</p>
        <div className="cta-buttons">
          <Link to="/signup" className="btn btn-primary btn-large">Get Started Free</Link>
          <Link to="/ai/matching" className="btn btn-secondary btn-large">Try AI Matching</Link>
        </div>
      </section>
    </div>
  )
}

export default Home
