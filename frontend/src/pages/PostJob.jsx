import { useState } from 'react'
import './PostJob.css'

function PostJob() {
  const [formData, setFormData] = useState({
    title: '',
    category: '',
    budget: '',
    description: '',
    skills: '',
    duration: ''
  })

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    })
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    console.log('Submitting job:', formData)
    alert('Job posted successfully! (Demo mode)')
    setFormData({
      title: '',
      category: '',
      budget: '',
      description: '',
      skills: '',
      duration: ''
    })
  }

  return (
    <div className="post-job-page">
      <div className="form-header">
        <h1>Post a New Job</h1>
        <p>Fill in the details to find the perfect freelancer</p>
      </div>

      <form className="job-form" onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="title">Job Title *</label>
          <input
            type="text"
            id="title"
            name="title"
            value={formData.title}
            onChange={handleChange}
            placeholder="e.g., Build a WordPress Website"
            required
          />
        </div>

        <div className="form-group">
          <label htmlFor="category">Category *</label>
          <select
            id="category"
            name="category"
            value={formData.category}
            onChange={handleChange}
            required
          >
            <option value="">Select a category</option>
            <option value="web">Web Development</option>
            <option value="mobile">Mobile Development</option>
            <option value="design">Design</option>
            <option value="writing">Writing</option>
            <option value="marketing">Marketing</option>
            <option value="data">Data Science</option>
          </select>
        </div>

        <div className="form-row">
          <div className="form-group">
            <label htmlFor="budget">Budget *</label>
            <input
              type="text"
              id="budget"
              name="budget"
              value={formData.budget}
              onChange={handleChange}
              placeholder="$1000 - $5000"
              required
            />
          </div>

          <div className="form-group">
            <label htmlFor="duration">Project Duration *</label>
            <select
              id="duration"
              name="duration"
              value={formData.duration}
              onChange={handleChange}
              required
            >
              <option value="">Select duration</option>
              <option value="1-week">Less than 1 week</option>
              <option value="1-month">1 Month</option>
              <option value="3-months">3 Months</option>
              <option value="6-months">6+ Months</option>
            </select>
          </div>
        </div>

        <div className="form-group">
          <label htmlFor="description">Project Description *</label>
          <textarea
            id="description"
            name="description"
            value={formData.description}
            onChange={handleChange}
            placeholder="Describe your project in detail..."
            rows="6"
            required
          />
        </div>

        <div className="form-group">
          <label htmlFor="skills">Required Skills</label>
          <input
            type="text"
            id="skills"
            name="skills"
            value={formData.skills}
            onChange={handleChange}
            placeholder="React, Node.js, MongoDB (comma separated)"
          />
        </div>

        <div className="form-actions">
          <button type="button" className="btn btn-secondary">Save as Draft</button>
          <button type="submit" className="btn btn-primary">Post Job</button>
        </div>
      </form>
    </div>
  )
}

export default PostJob
