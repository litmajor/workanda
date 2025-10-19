import { useState } from 'react'
import { useParams } from 'react-router-dom'
import './Profile.css'

function Profile() {
  const { id } = useParams()
  const [isEditing, setIsEditing] = useState(false)
  const [profile, setProfile] = useState({
    name: 'Alex Johnson',
    title: 'Full Stack Developer',
    location: 'San Francisco, CA',
    hourlyRate: '$75/hr',
    bio: 'Experienced full-stack developer with 5+ years building web applications. Specialized in React, Node.js, and cloud infrastructure.',
    skills: ['React', 'Node.js', 'TypeScript', 'AWS', 'MongoDB', 'Docker'],
    languages: ['English (Native)', 'Spanish (Fluent)'],
    trustScore: 92,
    completedJobs: 45,
    responseTime: '2 hours',
    successRate: 98
  })

  const [portfolio] = useState([
    {
      id: 1,
      title: 'E-commerce Platform',
      description: 'Built a full-featured e-commerce platform with React and Node.js',
      image: '🛒',
      tags: ['React', 'Node.js', 'Stripe']
    },
    {
      id: 2,
      title: 'Analytics Dashboard',
      description: 'Created real-time analytics dashboard with D3.js',
      image: '📊',
      tags: ['D3.js', 'React', 'WebSocket']
    },
    {
      id: 3,
      title: 'Mobile App Backend',
      description: 'Developed scalable REST API for fitness tracking app',
      image: '📱',
      tags: ['Node.js', 'MongoDB', 'AWS']
    }
  ])

  const [certifications] = useState([
    { name: 'AWS Certified Solutions Architect', issuer: 'Amazon', year: 2023 },
    { name: 'React Developer Certification', issuer: 'Meta', year: 2022 }
  ])

  const [earnings] = useState({
    total: '$45,680',
    thisMonth: '$3,200',
    lastMonth: '$4,100',
    avgPerProject: '$1,800'
  })

  const [availability] = useState({
    hoursPerWeek: 30,
    nextAvailable: 'Immediately',
    timezone: 'PST (UTC-8)'
  })

  return (
    <div className="profile-page">
      <div className="profile-container">
        <div className="profile-sidebar">
          <div className="profile-card">
            <div className="profile-avatar">
              <div className="avatar-circle">{profile.name[0]}</div>
              <div className="trust-score">
                <span className="score-value">{profile.trustScore}</span>
                <span className="score-label">Trust Score</span>
              </div>
            </div>
            
            <h2>{profile.name}</h2>
            <p className="profile-title">{profile.title}</p>
            <p className="profile-location">📍 {profile.location}</p>
            <p className="profile-rate">{profile.hourlyRate}</p>

            <div className="profile-stats">
              <div className="stat">
                <div className="stat-value">{profile.completedJobs}</div>
                <div className="stat-label">Jobs Done</div>
              </div>
              <div className="stat">
                <div className="stat-value">{profile.successRate}%</div>
                <div className="stat-label">Success Rate</div>
              </div>
            </div>

            <button className="btn btn-primary btn-block">Hire Me</button>
            <button className="btn btn-secondary btn-block">Send Message</button>
          </div>

          <div className="profile-card">
            <h3>Skills</h3>
            <div className="skills-list">
              {profile.skills.map((skill, index) => (
                <span key={index} className="skill-tag">{skill}</span>
              ))}
            </div>
          </div>

          <div className="profile-card">
            <h3>Languages</h3>
            <ul className="languages-list">
              {profile.languages.map((lang, index) => (
                <li key={index}>{lang}</li>
              ))}
            </ul>
          </div>

          <div className="profile-card">
            <h3>Availability</h3>
            <div className="availability-info">
              <p><strong>Hours/Week:</strong> {availability.hoursPerWeek}</p>
              <p><strong>Available:</strong> {availability.nextAvailable}</p>
              <p><strong>Timezone:</strong> {availability.timezone}</p>
            </div>
          </div>

          <div className="profile-card">
            <h3>Earnings Summary</h3>
            <div className="earnings-summary">
              <div className="earning-item">
                <span className="earning-label">Total Earned</span>
                <span className="earning-value">{earnings.total}</span>
              </div>
              <div className="earning-item">
                <span className="earning-label">This Month</span>
                <span className="earning-value">{earnings.thisMonth}</span>
              </div>
              <div className="earning-item">
                <span className="earning-label">Avg Per Project</span>
                <span className="earning-value">{earnings.avgPerProject}</span>
              </div>
            </div>
          </div>
        </div>

        <div className="profile-main">
          <div className="profile-section">
            <h3>About</h3>
            <p>{profile.bio}</p>
          </div>

          <div className="profile-section">
            <h3>Portfolio</h3>
            <div className="portfolio-grid">
              {portfolio.map(item => (
                <div key={item.id} className="portfolio-item">
                  <div className="portfolio-icon">{item.image}</div>
                  <h4>{item.title}</h4>
                  <p>{item.description}</p>
                  <div className="portfolio-tags">
                    {item.tags.map((tag, index) => (
                      <span key={index} className="tag">{tag}</span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="profile-section">
            <h3>Certifications & Education</h3>
            <div className="certifications-list">
              {certifications.map((cert, index) => (
                <div key={index} className="certification-item">
                  <div className="cert-icon">🎓</div>
                  <div className="cert-details">
                    <h4>{cert.name}</h4>
                    <p>{cert.issuer} • {cert.year}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="profile-section">
            <h3>Work History & Reviews</h3>
            <div className="reviews-list">
              <div className="review-item">
                <div className="review-header">
                  <div>
                    <strong>Sarah Chen</strong>
                    <div className="stars">⭐⭐⭐⭐⭐</div>
                  </div>
                  <span className="review-date">2 weeks ago</span>
                </div>
                <p className="review-text">
                  "Outstanding work! Alex delivered the project ahead of schedule and exceeded expectations. Highly recommended!"
                </p>
                <div className="review-meta">
                  <span>E-commerce Platform Development</span>
                  <span className="review-budget">$5,000</span>
                </div>
              </div>

              <div className="review-item">
                <div className="review-header">
                  <div>
                    <strong>Mike Rodriguez</strong>
                    <div className="stars">⭐⭐⭐⭐⭐</div>
                  </div>
                  <span className="review-date">1 month ago</span>
                </div>
                <p className="review-text">
                  "Great communication and technical skills. Will definitely hire again for future projects."
                </p>
                <div className="review-meta">
                  <span>API Development</span>
                  <span className="review-budget">$3,500</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Profile
