import { useState, useEffect } from 'react'
import './Jobs.css'

function Jobs() {
  const [jobs, setJobs] = useState([
    {
      id: 1,
      title: "Full Stack Developer for E-commerce Platform",
      budget: "$5000 - $8000",
      description: "Looking for an experienced developer to build a modern e-commerce platform with React and Node.js",
      category: "Web Development",
      posted: "2 hours ago",
      proposals: 12
    },
    {
      id: 2,
      title: "Mobile App UI/UX Designer",
      budget: "$2000 - $3000",
      description: "Need a creative designer for a fitness tracking mobile app",
      category: "Design",
      posted: "5 hours ago",
      proposals: 8
    },
    {
      id: 3,
      title: "Python Data Analyst",
      budget: "$3000 - $4000",
      description: "Analyze customer data and create insights dashboard",
      category: "Data Science",
      posted: "1 day ago",
      proposals: 15
    },
    {
      id: 4,
      title: "Content Writer for Tech Blog",
      budget: "$500 - $1000",
      description: "Write 10 SEO-optimized articles about AI and machine learning",
      category: "Writing",
      posted: "2 days ago",
      proposals: 20
    }
  ])

  const [filter, setFilter] = useState('all')

  const categories = ['all', 'Web Development', 'Design', 'Data Science', 'Writing']

  const filteredJobs = filter === 'all' 
    ? jobs 
    : jobs.filter(job => job.category === filter)

  return (
    <div className="jobs-page">
      <div className="jobs-header">
        <h1>Browse Available Jobs</h1>
        <p>Find your next opportunity from {jobs.length} active projects</p>
      </div>

      <div className="jobs-filters">
        {categories.map(category => (
          <button
            key={category}
            className={`filter-btn ${filter === category ? 'active' : ''}`}
            onClick={() => setFilter(category)}
          >
            {category}
          </button>
        ))}
      </div>

      <div className="jobs-grid">
        {filteredJobs.map(job => (
          <div key={job.id} className="job-card">
            <div className="job-header">
              <h3>{job.title}</h3>
              <span className="job-budget">{job.budget}</span>
            </div>
            <p className="job-description">{job.description}</p>
            <div className="job-meta">
              <span className="job-category">{job.category}</span>
              <span className="job-time">{job.posted}</span>
              <span className="job-proposals">{job.proposals} proposals</span>
            </div>
            <button className="btn btn-primary">Submit Proposal</button>
          </div>
        ))}
      </div>
    </div>
  )
}

export default Jobs
