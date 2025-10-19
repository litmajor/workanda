import { useState } from 'react'
import { Link } from 'react-router-dom'
import './Proposals.css'

function Proposals() {
  const [filter, setFilter] = useState('all')

  const proposals = [
    {
      id: 1,
      jobTitle: 'Full Stack Developer for E-commerce Platform',
      bidAmount: '$6000',
      deliveryTime: '8 weeks',
      status: 'pending',
      submittedOn: '2 days ago',
      clientName: 'Sarah Chen',
      coverLetter: 'I have 5+ years of experience building e-commerce platforms with React and Node.js. I can deliver a high-quality solution within your timeline.',
      jobBudget: '$5000 - $8000'
    },
    {
      id: 2,
      jobTitle: 'Mobile App UI/UX Design',
      bidAmount: '$2500',
      deliveryTime: '3 weeks',
      status: 'accepted',
      submittedOn: '1 week ago',
      clientName: 'Mike Rodriguez',
      coverLetter: 'I specialize in mobile UI/UX design with a focus on user-centered design principles. My portfolio includes several fitness apps.',
      jobBudget: '$2000 - $3000'
    },
    {
      id: 3,
      jobTitle: 'Python Data Analysis Dashboard',
      bidAmount: '$3500',
      deliveryTime: '6 weeks',
      status: 'rejected',
      submittedOn: '2 weeks ago',
      clientName: 'Emily Watson',
      coverLetter: 'Expert in Python data analysis and visualization. I can create an interactive dashboard using Plotly and Dash.',
      jobBudget: '$3000 - $4000'
    },
    {
      id: 4,
      jobTitle: 'React Developer for SaaS Application',
      bidAmount: '$7500',
      deliveryTime: '10 weeks',
      status: 'interview',
      submittedOn: '4 days ago',
      clientName: 'David Kim',
      coverLetter: 'I have extensive experience with React and building scalable SaaS applications. I can deliver a production-ready solution.',
      jobBudget: '$6000 - $10000'
    }
  ]

  const filteredProposals = filter === 'all'
    ? proposals
    : proposals.filter(p => p.status === filter)

  const getStatusBadge = (status) => {
    const statusConfig = {
      pending: { label: 'Pending', class: 'status-pending' },
      accepted: { label: 'Accepted', class: 'status-accepted' },
      rejected: { label: 'Declined', class: 'status-rejected' },
      interview: { label: 'Interview', class: 'status-interview' }
    }
    return statusConfig[status] || statusConfig.pending
  }

  return (
    <div className="proposals-page">
      <div className="proposals-container">
        <div className="proposals-header">
          <h1>My Proposals</h1>
          <p>Track all your submitted proposals in one place</p>
        </div>

        <div className="proposals-filters">
          <button
            className={`filter-btn ${filter === 'all' ? 'active' : ''}`}
            onClick={() => setFilter('all')}
          >
            All ({proposals.length})
          </button>
          <button
            className={`filter-btn ${filter === 'pending' ? 'active' : ''}`}
            onClick={() => setFilter('pending')}
          >
            Pending ({proposals.filter(p => p.status === 'pending').length})
          </button>
          <button
            className={`filter-btn ${filter === 'accepted' ? 'active' : ''}`}
            onClick={() => setFilter('accepted')}
          >
            Accepted ({proposals.filter(p => p.status === 'accepted').length})
          </button>
          <button
            className={`filter-btn ${filter === 'interview' ? 'active' : ''}`}
            onClick={() => setFilter('interview')}
          >
            Interview ({proposals.filter(p => p.status === 'interview').length})
          </button>
          <button
            className={`filter-btn ${filter === 'rejected' ? 'active' : ''}`}
            onClick={() => setFilter('rejected')}
          >
            Declined ({proposals.filter(p => p.status === 'rejected').length})
          </button>
        </div>

        <div className="proposals-list">
          {filteredProposals.map(proposal => {
            const statusBadge = getStatusBadge(proposal.status)
            return (
              <div key={proposal.id} className="proposal-card">
                <div className="proposal-header">
                  <div>
                    <Link to={`/jobs/${proposal.id}`} className="proposal-job-title">
                      {proposal.jobTitle}
                    </Link>
                    <p className="proposal-client">Client: {proposal.clientName}</p>
                  </div>
                  <span className={`status-badge ${statusBadge.class}`}>
                    {statusBadge.label}
                  </span>
                </div>

                <div className="proposal-details">
                  <div className="detail-row">
                    <span className="detail-label">Your Bid:</span>
                    <span className="detail-value bid-amount">{proposal.bidAmount}</span>
                  </div>
                  <div className="detail-row">
                    <span className="detail-label">Client Budget:</span>
                    <span className="detail-value">{proposal.jobBudget}</span>
                  </div>
                  <div className="detail-row">
                    <span className="detail-label">Delivery Time:</span>
                    <span className="detail-value">{proposal.deliveryTime}</span>
                  </div>
                  <div className="detail-row">
                    <span className="detail-label">Submitted:</span>
                    <span className="detail-value">{proposal.submittedOn}</span>
                  </div>
                </div>

                <div className="proposal-cover-letter">
                  <h4>Cover Letter</h4>
                  <p>{proposal.coverLetter}</p>
                </div>

                <div className="proposal-actions">
                  {proposal.status === 'pending' && (
                    <>
                      <button className="btn btn-secondary">Edit Proposal</button>
                      <button className="btn btn-danger">Withdraw</button>
                    </>
                  )}
                  {proposal.status === 'accepted' && (
                    <>
                      <Link to={`/projects/${proposal.id}`} className="btn btn-primary">
                        View Project
                      </Link>
                      <Link to={`/messages`} className="btn btn-secondary">
                        Message Client
                      </Link>
                    </>
                  )}
                  {proposal.status === 'interview' && (
                    <>
                      <button className="btn btn-primary">Schedule Interview</button>
                      <Link to={`/messages`} className="btn btn-secondary">
                        Message Client
                      </Link>
                    </>
                  )}
                  {proposal.status === 'rejected' && (
                    <button className="btn btn-secondary">View Feedback</button>
                  )}
                </div>
              </div>
            )
          })}
        </div>

        {filteredProposals.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">📝</div>
            <h3>No proposals found</h3>
            <p>Start submitting proposals to track them here</p>
            <Link to="/jobs" className="btn btn-primary">
              Browse Jobs
            </Link>
          </div>
        )}
      </div>
    </div>
  )
}

export default Proposals
