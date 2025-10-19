import { useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import './JobDetails.css'

function JobDetails() {
  const { id } = useParams()
  const [showProposalForm, setShowProposalForm] = useState(false)
  const [proposal, setProposal] = useState({
    coverLetter: '',
    bidAmount: '',
    deliveryTime: ''
  })

  const job = {
    id: id || 1,
    title: "Full Stack Developer for E-commerce Platform",
    budget: "$5000 - $8000",
    category: "Web Development",
    posted: "2 hours ago",
    proposals: 12,
    description: `We are looking for an experienced full-stack developer to build a modern e-commerce platform. 
    
The platform should include:
- User authentication and authorization
- Product catalog with search and filtering
- Shopping cart and checkout process
- Payment integration (Stripe)
- Admin dashboard for inventory management
- Order tracking and notifications
- Responsive design for mobile and desktop

The ideal candidate should have:
- 3+ years of experience with React and Node.js
- Strong knowledge of database design (MongoDB or PostgreSQL)
- Experience with payment gateway integration
- Understanding of security best practices
- Good communication skills

Timeline: 8-10 weeks
Budget: Negotiable based on experience`,
    skills: ['React', 'Node.js', 'MongoDB', 'Stripe', 'REST API', 'Docker'],
    client: {
      name: 'TechCorp Inc.',
      rating: 4.8,
      jobsPosted: 23,
      hireRate: 87,
      location: 'New York, USA'
    },
    scope: 'Large Project',
    duration: '2-3 months',
    experienceLevel: 'Expert',
    projectType: 'Fixed Price'
  }

  const handleProposalChange = (e) => {
    setProposal({
      ...proposal,
      [e.target.name]: e.target.value
    })
  }

  const handleSubmitProposal = (e) => {
    e.preventDefault()
    console.log('Submitting proposal:', proposal)
    alert('Proposal submitted successfully!')
    setShowProposalForm(false)
  }

  return (
    <div className="job-details-page">
      <div className="job-details-container">
        <div className="job-main">
          <div className="job-header-section">
            <div className="breadcrumb">
              <Link to="/jobs">Jobs</Link> / <span>{job.title}</span>
            </div>
            <h1>{job.title}</h1>
            <div className="job-meta-row">
              <span className="meta-item">📝 Posted {job.posted}</span>
              <span className="meta-item">📍 {job.client.location}</span>
              <span className="meta-item">💼 {job.proposals} proposals</span>
            </div>
          </div>

          <div className="job-content-section">
            <h2>Project Description</h2>
            <div className="job-description">
              {job.description.split('\n').map((paragraph, index) => (
                <p key={index}>{paragraph}</p>
              ))}
            </div>
          </div>

          <div className="job-content-section">
            <h2>Required Skills</h2>
            <div className="skills-list">
              {job.skills.map((skill, index) => (
                <span key={index} className="skill-badge">{skill}</span>
              ))}
            </div>
          </div>

          <div className="job-content-section">
            <h2>Project Details</h2>
            <div className="project-details-grid">
              <div className="detail-item">
                <span className="detail-label">Scope</span>
                <span className="detail-value">{job.scope}</span>
              </div>
              <div className="detail-item">
                <span className="detail-label">Duration</span>
                <span className="detail-value">{job.duration}</span>
              </div>
              <div className="detail-item">
                <span className="detail-label">Experience Level</span>
                <span className="detail-value">{job.experienceLevel}</span>
              </div>
              <div className="detail-item">
                <span className="detail-label">Project Type</span>
                <span className="detail-value">{job.projectType}</span>
              </div>
            </div>
          </div>

          {showProposalForm && (
            <div className="proposal-form-section">
              <h2>Submit Your Proposal</h2>
              <form onSubmit={handleSubmitProposal} className="proposal-form">
                <div className="form-group">
                  <label>Cover Letter</label>
                  <textarea
                    name="coverLetter"
                    value={proposal.coverLetter}
                    onChange={handleProposalChange}
                    rows="8"
                    placeholder="Explain why you're the best fit for this project..."
                    required
                  />
                </div>
                <div className="form-row">
                  <div className="form-group">
                    <label>Your Bid Amount</label>
                    <input
                      type="text"
                      name="bidAmount"
                      value={proposal.bidAmount}
                      onChange={handleProposalChange}
                      placeholder="$5000"
                      required
                    />
                  </div>
                  <div className="form-group">
                    <label>Delivery Time</label>
                    <select
                      name="deliveryTime"
                      value={proposal.deliveryTime}
                      onChange={handleProposalChange}
                      required
                    >
                      <option value="">Select timeline</option>
                      <option value="1-week">Less than 1 week</option>
                      <option value="2-weeks">2 weeks</option>
                      <option value="1-month">1 month</option>
                      <option value="2-months">2 months</option>
                      <option value="3-months">3+ months</option>
                    </select>
                  </div>
                </div>
                <div className="form-actions">
                  <button type="button" className="btn btn-secondary" onClick={() => setShowProposalForm(false)}>
                    Cancel
                  </button>
                  <button type="submit" className="btn btn-primary">
                    Submit Proposal
                  </button>
                </div>
              </form>
            </div>
          )}
        </div>

        <div className="job-sidebar">
          <div className="sidebar-card action-card">
            <div className="budget-display">
              <span className="budget-label">Budget</span>
              <span className="budget-value">{job.budget}</span>
            </div>
            {!showProposalForm && (
              <button className="btn btn-primary btn-block" onClick={() => setShowProposalForm(true)}>
                Submit Proposal
              </button>
            )}
            <button className="btn btn-secondary btn-block">
              Save Job
            </button>
          </div>

          <div className="sidebar-card">
            <h3>About the Client</h3>
            <div className="client-info">
              <div className="client-avatar">{job.client.name[0]}</div>
              <div>
                <h4>{job.client.name}</h4>
                <div className="client-rating">
                  ⭐ {job.client.rating} rating
                </div>
              </div>
            </div>
            <div className="client-stats">
              <div className="stat">
                <strong>{job.client.jobsPosted}</strong>
                <span>Jobs Posted</span>
              </div>
              <div className="stat">
                <strong>{job.client.hireRate}%</strong>
                <span>Hire Rate</span>
              </div>
            </div>
            <button className="btn btn-secondary btn-block">
              View Client Profile
            </button>
          </div>

          <div className="sidebar-card">
            <h3>Similar Jobs</h3>
            <div className="similar-jobs">
              <div className="similar-job-item">
                <h4>React Developer Needed</h4>
                <p className="similar-budget">$3000 - $5000</p>
              </div>
              <div className="similar-job-item">
                <h4>Node.js Backend API</h4>
                <p className="similar-budget">$2000 - $4000</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default JobDetails
