import { useState } from 'react'
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
      proposals: 12,
      skills: ['React', 'Node.js', 'MongoDB'],
      duration: '2-3 months',
      matchScore: 95,
      remote: true,
      experienceLevel: 'Expert'
    },
    {
      id: 2,
      title: "Mobile App UI/UX Designer",
      budget: "$2000 - $3000",
      description: "Need a creative designer for a fitness tracking mobile app",
      category: "Design",
      posted: "5 hours ago",
      proposals: 8,
      skills: ['Figma', 'UI Design', 'Mobile Design'],
      duration: '1 month',
      matchScore: 78,
      remote: true,
      experienceLevel: 'Intermediate'
    },
    {
      id: 3,
      title: "Python Data Analyst",
      budget: "$3000 - $4000",
      description: "Analyze customer data and create insights dashboard",
      category: "Data Science",
      posted: "1 day ago",
      proposals: 15,
      skills: ['Python', 'Pandas', 'Tableau'],
      duration: '1-2 months',
      matchScore: 88,
      remote: true,
      experienceLevel: 'Intermediate'
    },
    {
      id: 4,
      title: "Content Writer for Tech Blog",
      budget: "$500 - $1000",
      description: "Write 10 SEO-optimized articles about AI and machine learning",
      category: "Writing",
      posted: "2 days ago",
      proposals: 20,
      skills: ['SEO', 'Technical Writing', 'AI'],
      duration: '2 weeks',
      matchScore: 65,
      remote: true,
      experienceLevel: 'Entry Level'
    },
    {
      id: 5,
      title: "DevOps Engineer for Cloud Migration",
      budget: "$8000 - $12000",
      description: "Migrate existing infrastructure to AWS with CI/CD setup",
      category: "DevOps",
      posted: "3 days ago",
      proposals: 7,
      skills: ['AWS', 'Docker', 'Kubernetes'],
      duration: '3-4 months',
      matchScore: 82,
      remote: true,
      experienceLevel: 'Expert'
    }
  ])

  const [filter, setFilter] = useState('all')
  const [budgetRange, setBudgetRange] = useState('all')
  const [experienceLevel, setExperienceLevel] = useState('all')
  const [sortBy, setSortBy] = useState('matchScore')
  const [showFilters, setShowFilters] = useState(false)
  const [searchTerm, setSearchTerm] = useState('')

  const categories = ['all', 'Web Development', 'Design', 'Data Science', 'Writing', 'DevOps']

  const filteredJobs = jobs
    .filter(job => filter === 'all' || job.category === filter)
    .filter(job => budgetRange === 'all' || filterByBudget(job, budgetRange))
    .filter(job => experienceLevel === 'all' || job.experienceLevel === experienceLevel)
    .filter(job => searchTerm === '' || job.title.toLowerCase().includes(searchTerm.toLowerCase()))
    .sort((a, b) => {
      if (sortBy === 'matchScore') return b.matchScore - a.matchScore
      if (sortBy === 'newest') return new Date(b.posted) - new Date(a.posted)
      if (sortBy === 'budget') return parseInt(b.budget.split('-')[1]) - parseInt(a.budget.split('-')[1])
      return 0
    })

  function filterByBudget(job, range) {
    const max = parseInt(job.budget.split('-')[1].replace(/[^0-9]/g, ''))
    if (range === 'under2k') return max < 2000
    if (range === '2k-5k') return max >= 2000 && max < 5000
    if (range === '5k-10k') return max >= 5000 && max < 10000
    if (range === 'over10k') return max >= 10000
    return true
  }

  return (
    <div className="jobs-page">
      <div className="jobs-header">
        <h1>Browse Available Jobs</h1>
        <p>Find your next opportunity from {jobs.length} active projects</p>
      </div>

      <div className="search-and-filter">
        <div className="search-bar">
          <input
            type="text"
            placeholder="Search jobs by title, skills, or keywords..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="search-input"
          />
          <button className="btn btn-primary">Search</button>
        </div>

        <div className="filter-controls">
          <button 
            className="btn btn-secondary"
            onClick={() => setShowFilters(!showFilters)}
          >
            {showFilters ? 'Hide Filters' : 'Show Filters'} 🔽
          </button>
          <select value={sortBy} onChange={(e) => setSortBy(e.target.value)} className="sort-select">
            <option value="matchScore">Best Match</option>
            <option value="newest">Newest First</option>
            <option value="budget">Highest Budget</option>
          </select>
        </div>
      </div>

      {showFilters && (
        <div className="advanced-filters">
          <div className="filter-group">
            <label>Category</label>
            <div className="filter-buttons">
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
          </div>

          <div className="filter-group">
            <label>Budget Range</label>
            <div className="filter-buttons">
              <button className={`filter-btn ${budgetRange === 'all' ? 'active' : ''}`} onClick={() => setBudgetRange('all')}>All</button>
              <button className={`filter-btn ${budgetRange === 'under2k' ? 'active' : ''}`} onClick={() => setBudgetRange('under2k')}>Under $2K</button>
              <button className={`filter-btn ${budgetRange === '2k-5k' ? 'active' : ''}`} onClick={() => setBudgetRange('2k-5k')}>$2K - $5K</button>
              <button className={`filter-btn ${budgetRange === '5k-10k' ? 'active' : ''}`} onClick={() => setBudgetRange('5k-10k')}>$5K - $10K</button>
              <button className={`filter-btn ${budgetRange === 'over10k' ? 'active' : ''}`} onClick={() => setBudgetRange('over10k')}>$10K+</button>
            </div>
          </div>

          <div className="filter-group">
            <label>Experience Level</label>
            <div className="filter-buttons">
              <button className={`filter-btn ${experienceLevel === 'all' ? 'active' : ''}`} onClick={() => setExperienceLevel('all')}>All</button>
              <button className={`filter-btn ${experienceLevel === 'Entry Level' ? 'active' : ''}`} onClick={() => setExperienceLevel('Entry Level')}>Entry Level</button>
              <button className={`filter-btn ${experienceLevel === 'Intermediate' ? 'active' : ''}`} onClick={() => setExperienceLevel('Intermediate')}>Intermediate</button>
              <button className={`filter-btn ${experienceLevel === 'Expert' ? 'active' : ''}`} onClick={() => setExperienceLevel('Expert')}>Expert</button>
            </div>
          </div>
        </div>
      )}

      <div className="jobs-results-header">
        <h3>{filteredJobs.length} jobs found</h3>
      </div>

      <div className="jobs-grid">
        {filteredJobs.map(job => (
          <div key={job.id} className="job-card">
            <div className="job-header">
              <div>
                <h3>{job.title}</h3>
                <div className="job-badges">
                  <span className="badge badge-category">{job.category}</span>
                  <span className="badge badge-experience">{job.experienceLevel}</span>
                  {job.remote && <span className="badge badge-remote">🌍 Remote</span>}
                </div>
              </div>
              <div className="match-score">
                <div className="match-percentage">{job.matchScore}%</div>
                <div className="match-label">Match</div>
              </div>
            </div>
            <p className="job-description">{job.description}</p>
            <div className="job-skills">
              {job.skills.map((skill, index) => (
                <span key={index} className="skill-tag">{skill}</span>
              ))}
            </div>
            <div className="job-meta">
              <span className="job-budget">💰 {job.budget}</span>
              <span className="job-duration">⏱️ {job.duration}</span>
              <span className="job-time">🕐 {job.posted}</span>
            </div>
            <div className="job-footer">
              <span className="job-proposals">{job.proposals} proposals</span>
              <div className="job-actions">
                <button className="btn btn-secondary btn-small">Save</button>
                <button className="btn btn-primary">Submit Proposal</button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}

export default Jobs
