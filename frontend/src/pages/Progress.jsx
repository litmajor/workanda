
import { useState } from 'react'
import './Progress.css'

function Progress() {
  const [activeSection, setActiveSection] = useState('overview')

  const sections = {
    backend: [
      { name: 'Core Infrastructure', progress: 95, status: 'complete' },
      { name: 'User Management', progress: 90, status: 'complete' },
      { name: 'Job & Project Management', progress: 85, status: 'in-progress' },
      { name: 'Payment & Escrow', progress: 75, status: 'in-progress' },
      { name: 'AI Features', progress: 80, status: 'in-progress' },
      { name: 'Communication', progress: 70, status: 'in-progress' },
      { name: 'Team & Agency', progress: 85, status: 'in-progress' },
      { name: 'Admin & Moderation', progress: 80, status: 'in-progress' },
      { name: 'Reviews & Ratings', progress: 90, status: 'complete' }
    ],
    frontend: [
      { name: 'Core Setup', progress: 100, status: 'complete' },
      { name: 'Pages Implementation', progress: 100, status: 'complete' },
      { name: 'Components', progress: 100, status: 'complete' },
      { name: 'API Integration', progress: 45, status: 'in-progress' },
      { name: 'UI/UX Polish', progress: 55, status: 'in-progress' }
    ],
    deployment: [
      { name: 'Development Environment', progress: 100, status: 'complete' },
      { name: 'Docker Setup', progress: 60, status: 'in-progress' },
      { name: 'CI/CD Pipeline', progress: 0, status: 'planned' },
      { name: 'Production Deployment', progress: 0, status: 'planned' },
      { name: 'Monitoring & Logging', progress: 40, status: 'in-progress' }
    ]
  }

  const overallProgress = {
    backend: 78,
    frontend: 85,
    deployment: 40,
    combined: 81
  }

  const getStatusColor = (status) => {
    switch (status) {
      case 'complete': return 'var(--success-color)'
      case 'in-progress': return 'var(--warning-color)'
      case 'planned': return 'var(--text-secondary)'
      default: return 'var(--text-secondary)'
    }
  }

  const getStatusIcon = (status) => {
    switch (status) {
      case 'complete': return '✅'
      case 'in-progress': return '⚠️'
      case 'planned': return '📋'
      default: return '⏳'
    }
  }

  return (
    <div className="progress-page">
      <div className="progress-container">
        <div className="progress-header">
          <h1>📊 Project Progress Tracker</h1>
          <p>Real-time status of Workanda development</p>
        </div>

        <div className="overall-stats">
          <div className="stat-card-progress">
            <h3>Backend</h3>
            <div className="progress-circle">
              <svg viewBox="0 0 100 100">
                <circle cx="50" cy="50" r="45" fill="none" stroke="var(--border-color)" strokeWidth="8"/>
                <circle 
                  cx="50" 
                  cy="50" 
                  r="45" 
                  fill="none" 
                  stroke="var(--primary-color)" 
                  strokeWidth="8"
                  strokeDasharray={`${overallProgress.backend * 2.827} 282.7`}
                  transform="rotate(-90 50 50)"
                />
                <text x="50" y="55" textAnchor="middle" fontSize="20" fontWeight="bold" fill="var(--text-primary)">
                  {overallProgress.backend}%
                </text>
              </svg>
            </div>
          </div>

          <div className="stat-card-progress">
            <h3>Frontend</h3>
            <div className="progress-circle">
              <svg viewBox="0 0 100 100">
                <circle cx="50" cy="50" r="45" fill="none" stroke="var(--border-color)" strokeWidth="8"/>
                <circle 
                  cx="50" 
                  cy="50" 
                  r="45" 
                  fill="none" 
                  stroke="var(--secondary-color)" 
                  strokeWidth="8"
                  strokeDasharray={`${overallProgress.frontend * 2.827} 282.7`}
                  transform="rotate(-90 50 50)"
                />
                <text x="50" y="55" textAnchor="middle" fontSize="20" fontWeight="bold" fill="var(--text-primary)">
                  {overallProgress.frontend}%
                </text>
              </svg>
            </div>
          </div>

          <div className="stat-card-progress">
            <h3>Deployment</h3>
            <div className="progress-circle">
              <svg viewBox="0 0 100 100">
                <circle cx="50" cy="50" r="45" fill="none" stroke="var(--border-color)" strokeWidth="8"/>
                <circle 
                  cx="50" 
                  cy="50" 
                  r="45" 
                  fill="none" 
                  stroke="var(--warning-color)" 
                  strokeWidth="8"
                  strokeDasharray={`${overallProgress.deployment * 2.827} 282.7`}
                  transform="rotate(-90 50 50)"
                />
                <text x="50" y="55" textAnchor="middle" fontSize="20" fontWeight="bold" fill="var(--text-primary)">
                  {overallProgress.deployment}%
                </text>
              </svg>
            </div>
          </div>

          <div className="stat-card-progress featured">
            <h3>Overall</h3>
            <div className="progress-circle large">
              <svg viewBox="0 0 100 100">
                <circle cx="50" cy="50" r="45" fill="none" stroke="var(--border-color)" strokeWidth="8"/>
                <circle 
                  cx="50" 
                  cy="50" 
                  r="45" 
                  fill="none" 
                  stroke="var(--primary-color)" 
                  strokeWidth="8"
                  strokeDasharray={`${overallProgress.combined * 2.827} 282.7`}
                  transform="rotate(-90 50 50)"
                />
                <text x="50" y="55" textAnchor="middle" fontSize="24" fontWeight="bold" fill="var(--text-primary)">
                  {overallProgress.combined}%
                </text>
              </svg>
            </div>
          </div>
        </div>

        <div className="progress-tabs">
          <button 
            className={`tab ${activeSection === 'overview' ? 'active' : ''}`}
            onClick={() => setActiveSection('overview')}
          >
            Overview
          </button>
          <button 
            className={`tab ${activeSection === 'backend' ? 'active' : ''}`}
            onClick={() => setActiveSection('backend')}
          >
            Backend
          </button>
          <button 
            className={`tab ${activeSection === 'frontend' ? 'active' : ''}`}
            onClick={() => setActiveSection('frontend')}
          >
            Frontend
          </button>
          <button 
            className={`tab ${activeSection === 'deployment' ? 'active' : ''}`}
            onClick={() => setActiveSection('deployment')}
          >
            Deployment
          </button>
        </div>

        {activeSection === 'overview' && (
          <div className="progress-content">
            <h2>Project Overview</h2>
            <div className="overview-grid">
              <div className="overview-card">
                <h3>🎯 Current Phase</h3>
                <p className="phase-name">MVP Development</p>
                <p>Completing core features and AI integration</p>
              </div>
              <div className="overview-card">
                <h3>📅 Timeline</h3>
                <p className="phase-name">Q1 2025</p>
                <p>Target launch for beta testing</p>
              </div>
              <div className="overview-card">
                <h3>👥 Team Status</h3>
                <p className="phase-name">Active Development</p>
                <p>All systems operational</p>
              </div>
            </div>

            <div className="feature-highlights">
              <h3>✨ Completed Features</h3>
              <div className="highlights-grid">
                <div className="highlight">✅ Authentication & Authorization</div>
                <div className="highlight">✅ AI Matching Engine</div>
                <div className="highlight">✅ Escrow System</div>
                <div className="highlight">✅ Project Management</div>
                <div className="highlight">✅ Team & Agency Support</div>
                <div className="highlight">✅ Trust & Safety Features</div>
              </div>
            </div>
          </div>
        )}

        {activeSection !== 'overview' && (
          <div className="progress-content">
            <h2>{activeSection.charAt(0).toUpperCase() + activeSection.slice(1)} Progress</h2>
            <div className="progress-list">
              {sections[activeSection].map((item, index) => (
                <div key={index} className="progress-item">
                  <div className="progress-item-header">
                    <span className="status-icon">{getStatusIcon(item.status)}</span>
                    <h4>{item.name}</h4>
                    <span className="progress-percentage">{item.progress}%</span>
                  </div>
                  <div className="progress-bar-container">
                    <div 
                      className="progress-bar-fill" 
                      style={{ 
                        width: `${item.progress}%`,
                        backgroundColor: getStatusColor(item.status)
                      }}
                    />
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default Progress
