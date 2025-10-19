
import { useState } from 'react'
import './Analytics.css'
import { EarningsChart, ProjectDistributionChart } from '../components/Chart'

function Analytics() {
  const [timeRange, setTimeRange] = useState('30days')

  const stats = {
    totalEarnings: '$45,680',
    totalProjects: 28,
    averageRating: 4.8,
    successRate: 96
  }

  const earningsData = [
    { month: 'Jan', earnings: 3200 },
    { month: 'Feb', earnings: 4100 },
    { month: 'Mar', earnings: 3800 },
    { month: 'Apr', earnings: 5200 },
    { month: 'May', earnings: 4800 },
    { month: 'Jun', earnings: 6100 }
  ]

  const projectDistribution = [
    { name: 'Web Dev', value: 12 },
    { name: 'Mobile', value: 8 },
    { name: 'Design', value: 5 },
    { name: 'Marketing', value: 3 }
  ]

  return (
    <div className="analytics-page">
      <div className="analytics-container">
        <div className="analytics-header">
          <div>
            <h1>Analytics Dashboard</h1>
            <p>Track your performance and insights</p>
          </div>
          <select
            value={timeRange}
            onChange={(e) => setTimeRange(e.target.value)}
            className="time-range-select"
          >
            <option value="7days">Last 7 Days</option>
            <option value="30days">Last 30 Days</option>
            <option value="90days">Last 90 Days</option>
            <option value="year">This Year</option>
          </select>
        </div>

        <div className="stats-grid">
          <div className="stat-card">
            <div className="stat-icon">💰</div>
            <div className="stat-content">
              <h3>{stats.totalEarnings}</h3>
              <p>Total Earnings</p>
              <span className="stat-change positive">+12% from last month</span>
            </div>
          </div>
          <div className="stat-card">
            <div className="stat-icon">📊</div>
            <div className="stat-content">
              <h3>{stats.totalProjects}</h3>
              <p>Completed Projects</p>
              <span className="stat-change positive">+3 this month</span>
            </div>
          </div>
          <div className="stat-card">
            <div className="stat-icon">⭐</div>
            <div className="stat-content">
              <h3>{stats.averageRating}</h3>
              <p>Average Rating</p>
              <span className="stat-change">Excellent</span>
            </div>
          </div>
          <div className="stat-card">
            <div className="stat-icon">✅</div>
            <div className="stat-content">
              <h3>{stats.successRate}%</h3>
              <p>Success Rate</p>
              <span className="stat-change positive">+2% improvement</span>
            </div>
          </div>
        </div>

        <div className="charts-grid">
          <div className="chart-card">
            <h3>Earnings Overview</h3>
            <EarningsChart data={earningsData} />
          </div>
          <div className="chart-card">
            <h3>Project Distribution</h3>
            <ProjectDistributionChart data={projectDistribution} />
          </div>
        </div>

        <div className="insights-section">
          <h3>Key Insights</h3>
          <div className="insights-list">
            <div className="insight-item">
              <div className="insight-icon success">📈</div>
              <div className="insight-content">
                <h4>Strong Performance</h4>
                <p>Your project completion rate increased by 15% this month</p>
              </div>
            </div>
            <div className="insight-item">
              <div className="insight-icon warning">⚠️</div>
              <div className="insight-content">
                <h4>Response Time</h4>
                <p>Your average response time is 4 hours. Consider reducing to 2 hours for better client satisfaction</p>
              </div>
            </div>
            <div className="insight-item">
              <div className="insight-icon info">💡</div>
              <div className="insight-content">
                <h4>Popular Skills</h4>
                <p>React and Node.js are your most requested skills this month</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Analytics
