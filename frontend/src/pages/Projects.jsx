import { useState } from 'react'
import { Link, useParams } from 'react-router-dom'
import './Projects.css'

function Projects() {
  const { id } = useParams()
  const [activeTab, setActiveTab] = useState('overview')

  const projects = [
    {
      id: 1,
      title: 'E-commerce Platform Development',
      client: 'Sarah Chen',
      status: 'in-progress',
      progress: 65,
      budget: '$6000',
      startDate: 'Nov 1, 2025',
      deadline: 'Dec 15, 2025',
      description: 'Building a modern e-commerce platform with React and Node.js',
      milestones: [
        { id: 1, title: 'User Authentication', status: 'completed', payment: '$1500' },
        { id: 2, title: 'Product Catalog', status: 'in-progress', payment: '$1500' },
        { id: 3, title: 'Shopping Cart', status: 'pending', payment: '$1500' },
        { id: 4, title: 'Payment Integration', status: 'pending', payment: '$1500' }
      ]
    },
    {
      id: 2,
      title: 'Mobile App UI Design',
      client: 'Mike Rodriguez',
      status: 'review',
      progress: 90,
      budget: '$2500',
      startDate: 'Oct 15, 2025',
      deadline: 'Nov 30, 2025',
      description: 'Designing UI/UX for a fitness tracking mobile app',
      milestones: [
        { id: 1, title: 'Wireframes', status: 'completed', payment: '$625' },
        { id: 2, title: 'UI Design', status: 'completed', payment: '$875' },
        { id: 3, title: 'Prototype', status: 'completed', payment: '$625' },
        { id: 4, title: 'Final Delivery', status: 'in-progress', payment: '$375' }
      ]
    }
  ]

  const currentProject = id ? projects.find(p => p.id === parseInt(id)) : projects[0]

  if (!currentProject) {
    return <div className="projects-page"><p>Project not found</p></div>
  }

  const getStatusBadge = (status) => {
    const config = {
      'in-progress': { label: 'In Progress', class: 'status-in-progress' },
      'review': { label: 'Under Review', class: 'status-review' },
      'completed': { label: 'Completed', class: 'status-completed' },
      'pending': { label: 'Pending', class: 'status-pending' }
    }
    return config[status] || config.pending
  }

  return (
    <div className="projects-page">
      <div className="projects-container">
        <div className="projects-sidebar">
          <h3>Active Projects</h3>
          <div className="project-list">
            {projects.map(project => (
              <Link
                key={project.id}
                to={`/projects/${project.id}`}
                className={`project-list-item ${currentProject.id === project.id ? 'active' : ''}`}
              >
                <h4>{project.title}</h4>
                <div className="project-mini-progress">
                  <div className="progress-bar-mini">
                    <div className="progress-fill-mini" style={{ width: `${project.progress}%` }} />
                  </div>
                  <span>{project.progress}%</span>
                </div>
              </Link>
            ))}
          </div>
          <Link to="/jobs" className="btn btn-primary btn-block">
            Find New Projects
          </Link>
        </div>

        <div className="project-main">
          <div className="project-header">
            <div>
              <h1>{currentProject.title}</h1>
              <p className="project-client">Client: {currentProject.client}</p>
            </div>
            <span className={`status-badge ${getStatusBadge(currentProject.status).class}`}>
              {getStatusBadge(currentProject.status).label}
            </span>
          </div>

          <div className="project-stats-grid">
            <div className="stat-card">
              <span className="stat-label">Budget</span>
              <span className="stat-value">{currentProject.budget}</span>
            </div>
            <div className="stat-card">
              <span className="stat-label">Progress</span>
              <span className="stat-value">{currentProject.progress}%</span>
            </div>
            <div className="stat-card">
              <span className="stat-label">Start Date</span>
              <span className="stat-value">{currentProject.startDate}</span>
            </div>
            <div className="stat-card">
              <span className="stat-label">Deadline</span>
              <span className="stat-value">{currentProject.deadline}</span>
            </div>
          </div>

          <div className="project-tabs">
            <button
              className={`tab ${activeTab === 'overview' ? 'active' : ''}`}
              onClick={() => setActiveTab('overview')}
            >
              Overview
            </button>
            <button
              className={`tab ${activeTab === 'milestones' ? 'active' : ''}`}
              onClick={() => setActiveTab('milestones')}
            >
              Milestones
            </button>
            <button
              className={`tab ${activeTab === 'files' ? 'active' : ''}`}
              onClick={() => setActiveTab('files')}
            >
              Files
            </button>
            <button
              className={`tab ${activeTab === 'activity' ? 'active' : ''}`}
              onClick={() => setActiveTab('activity')}
            >
              Activity
            </button>
          </div>

          {activeTab === 'overview' && (
            <div className="project-section">
              <div className="section-card">
                <h3>Project Description</h3>
                <p>{currentProject.description}</p>
              </div>

              <div className="section-card">
                <h3>Overall Progress</h3>
                <div className="progress-section">
                  <div className="progress-bar-large">
                    <div
                      className="progress-fill-large"
                      style={{ width: `${currentProject.progress}%` }}
                    />
                  </div>
                  <span className="progress-text">{currentProject.progress}% Complete</span>
                </div>
              </div>

              <div className="section-card">
                <h3>Quick Actions</h3>
                <div className="action-buttons">
                  <button className="btn btn-primary">Submit Work</button>
                  <button className="btn btn-secondary">Request Payment</button>
                  <Link to="/messages" className="btn btn-secondary">Message Client</Link>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'milestones' && (
            <div className="project-section">
              <div className="milestones-list">
                {currentProject.milestones.map((milestone, index) => (
                  <div key={milestone.id} className="milestone-card">
                    <div className="milestone-header">
                      <div className="milestone-number">{index + 1}</div>
                      <div className="milestone-info">
                        <h4>{milestone.title}</h4>
                        <span className={`milestone-status ${milestone.status}`}>
                          {milestone.status.replace('-', ' ')}
                        </span>
                      </div>
                      <span className="milestone-payment">{milestone.payment}</span>
                    </div>
                    <div className="milestone-actions">
                      {milestone.status === 'in-progress' && (
                        <>
                          <button className="btn btn-small btn-primary">Submit for Review</button>
                          <button className="btn btn-small btn-secondary">Upload Files</button>
                        </>
                      )}
                      {milestone.status === 'completed' && (
                        <span className="completion-badge">✓ Completed & Paid</span>
                      )}
                      {milestone.status === 'pending' && (
                        <button className="btn btn-small btn-secondary">Start Working</button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === 'files' && (
            <div className="project-section">
              <div className="section-card">
                <div className="files-upload">
                  <button className="btn btn-primary">Upload Files</button>
                </div>
                <div className="files-list">
                  <div className="file-item">
                    <span className="file-icon">📄</span>
                    <div className="file-info">
                      <h4>project-requirements.pdf</h4>
                      <span>2.5 MB • Uploaded 5 days ago</span>
                    </div>
                    <button className="btn btn-small">Download</button>
                  </div>
                  <div className="file-item">
                    <span className="file-icon">🖼️</span>
                    <div className="file-info">
                      <h4>design-mockups.zip</h4>
                      <span>12.8 MB • Uploaded 3 days ago</span>
                    </div>
                    <button className="btn btn-small">Download</button>
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'activity' && (
            <div className="project-section">
              <div className="activity-timeline">
                <div className="activity-item">
                  <div className="activity-dot"></div>
                  <div className="activity-content">
                    <strong>Milestone 1 completed</strong>
                    <p>User Authentication module has been delivered and approved</p>
                    <span className="activity-time">2 days ago</span>
                  </div>
                </div>
                <div className="activity-item">
                  <div className="activity-dot"></div>
                  <div className="activity-content">
                    <strong>Payment released</strong>
                    <p>$1500 has been transferred to escrow</p>
                    <span className="activity-time">3 days ago</span>
                  </div>
                </div>
                <div className="activity-item">
                  <div className="activity-dot"></div>
                  <div className="activity-content">
                    <strong>Project started</strong>
                    <p>Contract signed and project kicked off</p>
                    <span className="activity-time">1 week ago</span>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default Projects
