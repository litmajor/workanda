
import { useState } from 'react'
import Chart from '../components/Chart'
import './TrustSafety.css'

function TrustSafety() {
  const [activeTab, setActiveTab] = useState('overview')

  const trustScore = {
    overall: 87,
    reliability: 92,
    communication: 85,
    quality: 90,
    professionalism: 88,
    transparency: 80,
    trend: 'Improving'
  }

  const fraudAlerts = [
    {
      id: 1,
      type: 'Unusual Behavior',
      severity: 'Low',
      description: 'Login from new location detected',
      date: '2025-01-15',
      status: 'Reviewed'
    }
  ]

  return (
    <div className="trust-safety-page">
      <div className="trust-container">
        <div className="trust-header">
          <div>
            <h1>Trust & Safety</h1>
            <p>Your security and reputation on the platform</p>
          </div>
          <div className="trust-score-badge">
            <div className="score-circle">
              <span className="score-value">{trustScore.overall}</span>
              <span className="score-label">Trust Score</span>
            </div>
            <span className={`trend-badge trend-${trustScore.trend.toLowerCase()}`}>
              {trustScore.trend === 'Improving' ? '↑' : trustScore.trend === 'Declining' ? '↓' : '→'} {trustScore.trend}
            </span>
          </div>
        </div>

        <div className="trust-tabs">
          <button
            className={`tab ${activeTab === 'overview' ? 'active' : ''}`}
            onClick={() => setActiveTab('overview')}
          >
            Overview
          </button>
          <button
            className={`tab ${activeTab === 'score' ? 'active' : ''}`}
            onClick={() => setActiveTab('score')}
          >
            Trust Score
          </button>
          <button
            className={`tab ${activeTab === 'security' ? 'active' : ''}`}
            onClick={() => setActiveTab('security')}
          >
            Security
          </button>
          <button
            className={`tab ${activeTab === 'alerts' ? 'active' : ''}`}
            onClick={() => setActiveTab('alerts')}
          >
            Alerts
          </button>
        </div>

        {activeTab === 'overview' && (
          <div className="trust-content">
            <div className="overview-grid">
              <div className="overview-card">
                <h3>🛡️ Account Security</h3>
                <div className="security-items">
                  <div className="security-item">
                    <span>✅ Two-Factor Authentication</span>
                    <span className="status-enabled">Enabled</span>
                  </div>
                  <div className="security-item">
                    <span>✅ Email Verified</span>
                    <span className="status-enabled">Verified</span>
                  </div>
                  <div className="security-item">
                    <span>✅ Phone Verified</span>
                    <span className="status-enabled">Verified</span>
                  </div>
                </div>
              </div>

              <div className="overview-card">
                <h3>📊 Reputation Metrics</h3>
                <div className="metrics-grid">
                  <div className="metric">
                    <span className="metric-value">98%</span>
                    <span className="metric-label">Project Completion</span>
                  </div>
                  <div className="metric">
                    <span className="metric-value">4.9/5</span>
                    <span className="metric-label">Client Rating</span>
                  </div>
                  <div className="metric">
                    <span className="metric-value">2.1h</span>
                    <span className="metric-label">Avg Response</span>
                  </div>
                </div>
              </div>
            </div>

            <div className="section-card">
              <h3>Recent Activity</h3>
              <div className="activity-list">
                <div className="activity-item">
                  <div className="activity-icon">🔐</div>
                  <div className="activity-content">
                    <strong>Login from new device</strong>
                    <p>MacBook Pro • San Francisco, CA</p>
                    <span className="activity-time">2 hours ago</span>
                  </div>
                </div>
                <div className="activity-item">
                  <div className="activity-icon">✅</div>
                  <div className="activity-content">
                    <strong>Project completed successfully</strong>
                    <p>E-commerce Platform Development</p>
                    <span className="activity-time">1 day ago</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'score' && (
          <div className="trust-content">
            <div className="section-card">
              <h3>Trust Score Breakdown</h3>
              <div className="score-components">
                {Object.entries(trustScore).filter(([key]) => !['overall', 'trend'].includes(key)).map(([key, value]) => (
                  <div key={key} className="score-component">
                    <div className="component-header">
                      <span className="component-name">{key.charAt(0).toUpperCase() + key.slice(1)}</span>
                      <span className="component-value">{value}</span>
                    </div>
                    <div className="component-bar">
                      <div className="component-fill" style={{ width: `${value}%` }} />
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <div className="section-card">
              <h3>Score Trend</h3>
              <Chart
                data={[
                  { month: 'Jan', score: 78 },
                  { month: 'Feb', score: 81 },
                  { month: 'Mar', score: 83 },
                  { month: 'Apr', score: 87 }
                ]}
                dataKey="score"
                color="#4f46e5"
              />
            </div>

            <div className="section-card">
              <h3>How to Improve Your Score</h3>
              <ul className="improvement-list">
                <li>✅ Complete projects on time (Current: 98%)</li>
                <li>✅ Respond to messages within 6 hours (Current: 2.1h)</li>
                <li>⚠️ Maintain quality consistency (Target: 95%+)</li>
                <li>✅ Keep budgets transparent and accurate</li>
              </ul>
            </div>
          </div>
        )}

        {activeTab === 'security' && (
          <div className="trust-content">
            <div className="section-card">
              <h3>Security Settings</h3>
              <div className="security-settings">
                <div className="setting-item">
                  <div className="setting-info">
                    <h4>Two-Factor Authentication</h4>
                    <p>Add an extra layer of security to your account</p>
                  </div>
                  <button className="btn btn-secondary">Enabled</button>
                </div>
                <div className="setting-item">
                  <div className="setting-info">
                    <h4>Login Notifications</h4>
                    <p>Get notified when someone logs into your account</p>
                  </div>
                  <label className="toggle-switch">
                    <input type="checkbox" defaultChecked />
                    <span className="toggle-slider"></span>
                  </label>
                </div>
                <div className="setting-item">
                  <div className="setting-info">
                    <h4>Session Management</h4>
                    <p>Manage active sessions and devices</p>
                  </div>
                  <button className="btn btn-secondary">Manage</button>
                </div>
              </div>
            </div>

            <div className="section-card">
              <h3>Recent Login Activity</h3>
              <div className="login-history">
                <div className="login-item">
                  <div className="login-icon">💻</div>
                  <div className="login-details">
                    <strong>MacBook Pro</strong>
                    <p>San Francisco, CA • 192.168.1.1</p>
                    <span>2 hours ago</span>
                  </div>
                  <span className="login-status current">Current</span>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'alerts' && (
          <div className="trust-content">
            <div className="section-card">
              <h3>Fraud Alerts</h3>
              {fraudAlerts.length > 0 ? (
                <div className="alerts-list">
                  {fraudAlerts.map(alert => (
                    <div key={alert.id} className={`alert-item severity-${alert.severity.toLowerCase()}`}>
                      <div className="alert-header">
                        <span className="alert-type">{alert.type}</span>
                        <span className={`alert-severity ${alert.severity.toLowerCase()}`}>{alert.severity}</span>
                      </div>
                      <p>{alert.description}</p>
                      <div className="alert-footer">
                        <span>{alert.date}</span>
                        <span className="alert-status">{alert.status}</span>
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="empty-alerts">
                  <span className="empty-icon">🎉</span>
                  <p>No security alerts. Your account is safe!</p>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default TrustSafety
