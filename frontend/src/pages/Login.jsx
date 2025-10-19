import { useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import './Auth.css'

function Login() {
  const navigate = useNavigate()
  const [formData, setFormData] = useState({
    email: '',
    password: ''
  })
  const [error, setError] = useState('')

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    })
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    setError('')
    
    // TODO: Implement actual API call
    console.log('Login attempt:', formData)
    
    // Demo: simulate successful login
    if (formData.email && formData.password) {
      localStorage.setItem('user', JSON.stringify({ email: formData.email }))
      navigate('/dashboard')
    } else {
      setError('Please fill in all fields')
    }
  }

  return (
    <div className="auth-page">
      <div className="auth-container">
        <div className="auth-card">
          <div className="auth-header">
            <h1>Welcome Back</h1>
            <p>Log in to continue to Workanda</p>
          </div>

          {error && (
            <div className="alert alert-error">
              {error}
            </div>
          )}

          <form onSubmit={handleSubmit} className="auth-form">
            <div className="form-group">
              <label htmlFor="email">Email Address</label>
              <input
                type="email"
                id="email"
                name="email"
                value={formData.email}
                onChange={handleChange}
                placeholder="your@email.com"
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="password">Password</label>
              <input
                type="password"
                id="password"
                name="password"
                value={formData.password}
                onChange={handleChange}
                placeholder="••••••••"
                required
              />
            </div>

            <div className="form-footer">
              <label className="checkbox-label">
                <input type="checkbox" /> Remember me
              </label>
              <Link to="/forgot-password" className="link">Forgot password?</Link>
            </div>

            <button type="submit" className="btn btn-primary btn-block">
              Log In
            </button>
          </form>

          <div className="auth-divider">
            <span>OR</span>
          </div>

          <button className="btn btn-secondary btn-block">
            <span>🔐</span> Continue with Google
          </button>

          <p className="auth-switch">
            Don't have an account? <Link to="/signup" className="link">Sign up</Link>
          </p>
        </div>
      </div>
    </div>
  )
}

export default Login
