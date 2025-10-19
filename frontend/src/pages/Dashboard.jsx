import { useState } from 'react'
import './Dashboard.css'

function Dashboard() {
  const [activeTab, setActiveTab] = useState('overview')

  const stats = {
    activeProjects: 3,
    completedProjects: 12,
    totalEarnings: '$15,420',
    pendingPayments: '$2,500'
  }

  const activeProjects = [
    {
      id: 1,
      title: "E-commerce Website Development",
      client: "TechCorp Inc.",
      status: "In Progress",
      progress: 65,
      deadline: "Dec 15, 2025",
      amount: "$5,000"
    },
    {
      id: 2,
      title: "Mobile App UI Design",
      client: "StartupX",
      status: "Review",
      progress: 90,
      deadline: "Nov 30, 2025",
      amount: "$2,000"
    },
    {
      id: 3,
      title: "Data Analysis Dashboard",
      client: "Analytics Co.",
      status: "In Progress",
      progress: 35,
      deadline: "Jan 10, 2026",
      amount: "$3,500"
    }
  ]

  const proposals = [
    {
      id: 1,
      job: "Python Backend Developer",
      bidAmount: "$4,000",
      status: "Pending",
      submittedOn: "Oct 15, 2025"
    },
    {
      id: 2,
      job: "React Developer for SaaS",
      bidAmount: "$6,000",
      status: "Accepted",
      submittedOn: "Oct 10, 2025"
    }
  ]

  return (
    <div className="dashboard">
      <div className="dashboard-header">
        <h1>Welcome back, Freelancer!</h1>
        <p>Here's what's happening with your projects</p>
      </div>

      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-value">{stats.activeProjects}</div>
          <div className="stat-label">Active Projects</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{stats.completedProjects}</div>
          <div className="stat-label">Completed</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{stats.totalEarnings}</div>
          <div className="stat-label">Total Earnings</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{stats.pendingPayments}</div>
          <div className="stat-label">Pending Payments</div>
        </div>
      </div>

      <div className="dashboard-tabs">
        <button 
          className={`tab ${activeTab === 'overview' ? 'active' : ''}`}
          onClick={() => setActiveTab('overview')}
        >
          Overview
        </button>
        <button 
          className={`tab ${activeTab === 'proposals' ? 'active' : ''}`}
          onClick={() => setActiveTab('proposals')}
        >
          Proposals
        </button>
      </div>

      {activeTab === 'overview' && (
        <div className="projects-section">
          <h2>Active Projects</h2>
          <div className="projects-list">
            {activeProjects.map(project => (
              <div key={project.id} className="project-card">
                <div className="project-header">
                  <div>
                    <h3>{project.title}</h3>
                    <p className="project-client">{project.client}</p>
                  </div>
                  <span className={`status-badge ${project.status.toLowerCase().replace(' ', '-')}`}>
                    {project.status}
                  </span>
                </div>
                <div className="project-progress">
                  <div className="progress-bar">
                    <div 
                      className="progress-fill" 
                      style={{ width: `${project.progress}%` }}
                    />
                  </div>
                  <span className="progress-text">{project.progress}% complete</span>
                </div>
                <div className="project-footer">
                  <span>Deadline: {project.deadline}</span>
                  <span className="project-amount">{project.amount}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'proposals' && (
        <div className="proposals-section">
          <h2>My Proposals</h2>
          <div className="proposals-table">
            <table>
              <thead>
                <tr>
                  <th>Job Title</th>
                  <th>Bid Amount</th>
                  <th>Status</th>
                  <th>Submitted On</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {proposals.map(proposal => (
                  <tr key={proposal.id}>
                    <td>{proposal.job}</td>
                    <td>{proposal.bidAmount}</td>
                    <td>
                      <span className={`status-badge ${proposal.status.toLowerCase()}`}>
                        {proposal.status}
                      </span>
                    </td>
                    <td>{proposal.submittedOn}</td>
                    <td>
                      <button className="btn-small">View</button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  )
}

export default Dashboard
