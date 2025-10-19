
import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useApp } from '../context/AppContext'
import { authAPI } from '../services/api'

export const useAuth = () => {
  const { login: setUser, logout: clearUser, addAlert } = useApp()
  const navigate = useNavigate()
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)

  const login = async (email, password) => {
    setLoading(true)
    setError(null)
    try {
      const response = await authAPI.login(email, password)
      setUser(response.user)
      addAlert('success', 'Login successful!')
      navigate('/dashboard')
      return response
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Login failed')
      throw err
    } finally {
      setLoading(false)
    }
  }

  const signup = async (userData) => {
    setLoading(true)
    setError(null)
    try {
      const response = await authAPI.signup(userData)
      setUser(response.user)
      addAlert('success', 'Account created successfully!')
      navigate('/dashboard')
      return response
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Signup failed')
      throw err
    } finally {
      setLoading(false)
    }
  }

  const logout = () => {
    authAPI.logout()
    clearUser()
    addAlert('info', 'Logged out successfully')
    navigate('/login')
  }

  const verifyMFA = async (code) => {
    setLoading(true)
    setError(null)
    try {
      const response = await authAPI.verifyMFA(code)
      addAlert('success', 'MFA verified successfully!')
      return response
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'MFA verification failed')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return {
    login,
    signup,
    logout,
    verifyMFA,
    loading,
    error
  }
}
