import { useState } from 'react'
import './AdminDashboard.css'

function AdminDashboard() {
  const [activeTab, setActiveTab] = useState('overview')

  const stats = {
    totalUsers: 15420,
    activeProjects: 2341,
    totalRevenue: '$245,680',
    platformFees: '$12,284',
    newUsersToday: 127,
    projectsCompleted: 8934
  }

  const recentUsers = [
    { id: 1, name: 'Alice Johnson', email: 'alice@example.com', role: 'Freelancer', joined: '2 hours ago', status: 'active' },
    { id: 2, name: 'Bob Smith', email: 'bob@example.com', role: 'Client', joined: '5 hours ago', status: 'active' },
    { id: 3, name: 'Carol Davis', email: 'carol@example.com', role: 'Freelancer', joined: '1 day ago', status: 'pending' }
  ]

  const recentTransactions = [
    { id: 1, from: 'Sarah Chen', to: 'Alex Johnson', amount: '$1500', type: 'Milestone Payment', status: 'completed', date: '1 hour ago' },
    { id: 2, from: 'Mike Rodriguez', to: 'Platform', amount: '$75', type: 'Service Fee', status: 'completed', date: '3 hours ago' },
    { id: 3, from: 'Emily Watson', to: 'Escrow', amount: '$3500', type: 'Project Deposit', status: 'pending', date: '5 hours ago' }
  ]

  const flaggedContent = [
    { id: 1, type: 'Job Post', title: 'Suspicious Job Posting', reporter: 'User #1234', reason: 'Spam', date: '2 hours ago' },
    { id: 2, type: 'Review', title: 'Fake Review Report', reporter: 'User #5678', reason: 'Fake', date: '4 hours ago' }
  ]

  return (
    <div className="admin-dashboard-page">
      <div className="admin-container">
        <div className="admin-header">
          <h1>Admin Dashboard</h1>
          <p>Platform overview and management</p>
        </div>

        <div className="stats-grid">
          <div className="stat-card">
            <div className="stat-icon">👥</div>
            <div className="stat-content">
              <h3>{stats.totalUsers.toLocaleString()}</h3>
              <p>Total Users</p>
              <span className="stat-change positive">+{stats.newUsersToday} today</span>
            </div>
          </div>
          <div className="stat-card">
            <div className="stat-icon">💼</div>
            <div className="stat-content">
              <h3>{stats.activeProjects.toLocaleString()}</h3>
              <p>Active Projects</p>
              <span className="stat-change">Live now</span>
            </div>
          </div>
          <div className="stat-card">
            <div className="stat-icon">💰</div>
            <div className="stat-content">
              <h3>{stats.totalRevenue}</h3>
              <p>Total Revenue</p>
              <span className="stat-change positive">+15% this month</span>
            </div>
          </div>
          <div className="stat-card">
            <div className="stat-icon">✅</div>
            <div className="stat-content">
              <h3>{stats.projectsCompleted.toLocaleString()}</h3>
              <p>Completed Projects</p>
              <span className="stat-change">All time</span>
            </div>
          </div>
        </div>

        <div className="admin-tabs">
          <button className={`tab ${activeTab === 'overview' ? 'active' : ''}`} onClick={() => setActiveTab('overview')}>
            Overview
          </button>
          <button className={`tab ${activeTab === 'users' ? 'active' : ''}`} onClick={() => setActiveTab('users')}>
            Users
          </button>
          <button className={`tab ${activeTab === 'transactions' ? 'active' : ''}`} onClick={() => setActiveTab('transactions')}>
            Transactions
          </button>
          <button className={`tab ${activeTab === 'moderation' ? 'active' : ''}`} onClick={() => setActiveTab('moderation')}>
            Moderation
          </button>
        </div>

        {activeTab === 'overview' && (
          <div className="admin-section">
            <div className="section-grid">
              <div className="chart-card">
                <h3>Revenue Trend</h3>
                <div className="chart-placeholder">
                  <p>📈 Revenue chart would go here</p>
                  <p className="chart-note">Integration with charting library needed</p>
                </div>
              </div>
              <div className="chart-card">
                <h3>User Growth</h3>
                <div className="chart-placeholder">
                  <p>📊 User growth chart would go here</p>
                  <p className="chart-note">Integration with charting library needed</p>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'users' && (
          <div className="admin-section">
            <div className="section-card">
              <div className="section-header">
                <h3>Recent Users</h3>
                <button className="btn btn-primary btn-small">Export CSV</button>
              </div>
              <div className="table-container">
                <table className="admin-table">
                  <thead>
                    <tr>
                      <th>Name</th>
                      <th>Email</th>
                      <th>Role</th>
                      <th>Joined</th>
                      <th>Status</th>
                      <th>Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {recentUsers.map(user => (
                      <tr key={user.id}>
                        <td>{user.name}</td>
                        <td>{user.email}</td>
                        <td><span className="role-badge">{user.role}</span></td>
                        <td>{user.joined}</td>
                        <td><span className={`status-badge ${user.status}`}>{user.status}</span></td>
                        <td>
                          <button className="btn btn-small">View</button>
                          <button className="btn btn-small btn-danger">Suspend</button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'transactions' && (
          <div className="admin-section">
            <div className="section-card">
              <div className="section-header">
                <h3>Recent Transactions</h3>
                <button className="btn btn-primary btn-small">View All</button>
              </div>
              <div className="table-container">
                <table className="admin-table">
                  <thead>
                    <tr>
                      <th>From</th>
                      <th>To</th>
                      <th>Amount</th>
                      <th>Type</th>
                      <th>Status</th>
                      <th>Date</th>
                      <th>Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {recentTransactions.map(transaction => (
                      <tr key={transaction.id}>
                        <td>{transaction.from}</td>
                        <td>{transaction.to}</td>
                        <td><strong>{transaction.amount}</strong></td>
                        <td>{transaction.type}</td>
                        <td><span className={`status-badge ${transaction.status}`}>{transaction.status}</span></td>
                        <td>{transaction.date}</td>
                        <td>
                          <button className="btn btn-small">Details</button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'moderation' && (
          <div className="admin-section">
            <div className="section-card">
              <div className="section-header">
                <h3>Flagged Content</h3>
                <span className="alert-badge">2 pending</span>
              </div>
              <div className="flagged-list">
                {flaggedContent.map(item => (
                  <div key={item.id} className="flagged-item">
                    <div className="flagged-info">
                      <h4>{item.title}</h4>
                      <div className="flagged-meta">
                        <span className="type-badge">{item.type}</span>
                        <span>Reported by {item.reporter}</span>
                        <span>Reason: {item.reason}</span>
                        <span>{item.date}</span>
                      </div>
                    </div>
                    <div className="flagged-actions">
                      <button className="btn btn-small btn-success">Approve</button>
                      <button className="btn btn-small btn-danger">Remove</button>
                      <button className="btn btn-small">Review</button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default AdminDashboard
