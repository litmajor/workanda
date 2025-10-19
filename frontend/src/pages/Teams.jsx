
import { useState } from 'react'
import Chart from '../components/Chart'
import Modal from '../components/Modal'
import './Teams.css'

function Teams() {
  const [activeTab, setActiveTab] = useState('my-teams')
  const [showCreateModal, setShowCreateModal] = useState(false)
  const [selectedTeam, setSelectedTeam] = useState(null)

  const teams = [
    {
      id: 1,
      name: 'Web Development Squad',
      members: 5,
      projects: 3,
      role: 'lead',
      avatar: 'W'
    },
    {
      id: 2,
      name: 'Design Team',
      members: 3,
      projects: 2,
      role: 'member',
      avatar: 'D'
    }
  ]

  const teamMembers = [
    { id: 1, name: 'Alice Johnson', role: 'Lead Developer', avatar: 'A', skills: ['React', 'Node.js'] },
    { id: 2, name: 'Bob Smith', role: 'Backend Developer', avatar: 'B', skills: ['Python', 'Django'] },
    { id: 3, name: 'Carol Davis', role: 'Designer', avatar: 'C', skills: ['Figma', 'UI/UX'] },
    { id: 4, name: 'David Kim', role: 'Frontend Developer', avatar: 'D', skills: ['Vue', 'CSS'] },
    { id: 5, name: 'Emma Wilson', role: 'QA Engineer', avatar: 'E', skills: ['Testing', 'Automation'] }
  ]

  return (
    <div className="teams-page">
      <div className="teams-container">
        <div className="teams-header">
          <div>
            <h1>Teams</h1>
            <p>Collaborate with your team members</p>
          </div>
          <button className="btn btn-primary" onClick={() => setShowCreateModal(true)}>Create New Team</button>
        </div>

        <div className="teams-tabs">
          <button
            className={`tab ${activeTab === 'my-teams' ? 'active' : ''}`}
            onClick={() => setActiveTab('my-teams')}
          >
            My Teams
          </button>
          <button
            className={`tab ${activeTab === 'invitations' ? 'active' : ''}`}
            onClick={() => setActiveTab('invitations')}
          >
            Invitations
          </button>
          <button
            className={`tab ${activeTab === 'analytics' ? 'active' : ''}`}
            onClick={() => setActiveTab('analytics')}
          >
            Analytics
          </button>
        </div>

        {activeTab === 'my-teams' && (
          <div className="teams-content">
            <div className="teams-grid">
              {teams.map(team => (
                <div key={team.id} className="team-card">
                  <div className="team-header">
                    <div className="team-avatar">{team.avatar}</div>
                    <div className="team-info">
                      <h3>{team.name}</h3>
                      <span className="team-role">{team.role}</span>
                    </div>
                  </div>
                  <div className="team-stats">
                    <div className="stat">
                      <span className="stat-value">{team.members}</span>
                      <span className="stat-label">Members</span>
                    </div>
                    <div className="stat">
                      <span className="stat-value">{team.projects}</span>
                      <span className="stat-label">Projects</span>
                    </div>
                  </div>
                  <div className="team-actions">
                    <button className="btn btn-primary btn-small">View Team</button>
                    <button className="btn btn-secondary btn-small">Settings</button>
                  </div>
                </div>
              ))}
            </div>

            <div className="section-card">
              <h3>Team Members</h3>
              <div className="members-list">
                {teamMembers.map(member => (
                  <div key={member.id} className="member-item">
                    <div className="member-avatar">{member.avatar}</div>
                    <div className="member-info">
                      <h4>{member.name}</h4>
                      <p>{member.role}</p>
                      <div className="member-skills">
                        {member.skills.map((skill, idx) => (
                          <span key={idx} className="skill-tag">{skill}</span>
                        ))}
                      </div>
                    </div>
                    <button className="btn btn-secondary btn-small">Message</button>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {activeTab === 'invitations' && (
          <div className="teams-content">
            <div className="empty-state">
              <div className="empty-icon">📬</div>
              <h3>No team invitations</h3>
              <p>You don't have any pending team invitations</p>
            </div>
          </div>
        )}

        {activeTab === 'analytics' && (
          <div className="teams-content">
            <div className="section-card">
              <h3>Team Performance</h3>
              <Chart
                data={[
                  { name: 'Jan', projects: 4, revenue: 12000 },
                  { name: 'Feb', projects: 5, revenue: 15000 },
                  { name: 'Mar', projects: 6, revenue: 18000 },
                  { name: 'Apr', projects: 8, revenue: 24000 }
                ]}
                dataKey="revenue"
                color="#10b981"
              />
            </div>

            <div className="stats-grid">
              <div className="stat-card">
                <span className="stat-label">Total Projects</span>
                <span className="stat-value">23</span>
              </div>
              <div className="stat-card">
                <span className="stat-label">Success Rate</span>
                <span className="stat-value">94%</span>
              </div>
              <div className="stat-card">
                <span className="stat-label">Avg Response Time</span>
                <span className="stat-value">2.4h</span>
              </div>
              <div className="stat-card">
                <span className="stat-label">Client Satisfaction</span>
                <span className="stat-value">4.8/5</span>
              </div>
            </div>
          </div>
        )}
      </div>

      {showCreateModal && (
        <Modal onClose={() => setShowCreateModal(false)}>
          <h2>Create New Team</h2>
          <form className="team-form">
            <div className="form-group">
              <label>Team Name</label>
              <input type="text" placeholder="Enter team name" required />
            </div>
            <div className="form-group">
              <label>Description</label>
              <textarea placeholder="What does your team specialize in?" rows="3"></textarea>
            </div>
            <div className="form-group">
              <label>Skills</label>
              <input type="text" placeholder="e.g., React, Node.js, UI/UX" />
            </div>
            <div className="modal-actions">
              <button type="button" className="btn btn-secondary" onClick={() => setShowCreateModal(false)}>
                Cancel
              </button>
              <button type="submit" className="btn btn-primary">
                Create Team
              </button>
            </div>
          </form>
        </Modal>
      )}
    </div>
  )
}

export default Teams
