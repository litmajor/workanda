
import { useState } from 'react'
import { aiAPI } from '../services/api'
import { useApp } from '../context/AppContext'

export const useAIMatching = () => {
  const [matches, setMatches] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const getJobMatches = async (jobId) => {
    setLoading(true)
    setError(null)
    try {
      const data = await aiAPI.getJobMatches(jobId)
      setMatches(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to get matches')
      throw err
    } finally {
      setLoading(false)
    }
  }

  const getFreelancerMatches = async (freelancerId) => {
    setLoading(true)
    setError(null)
    try {
      const data = await aiAPI.getFreelancerMatches(freelancerId)
      setMatches(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to get matches')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { matches, getJobMatches, getFreelancerMatches, loading, error }
}

export const useAITeamSuggestion = () => {
  const [suggestion, setSuggestion] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const suggestTeam = async (projectRequirements) => {
    setLoading(true)
    setError(null)
    try {
      const data = await aiAPI.suggestTeam(projectRequirements)
      setSuggestion(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to suggest team')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { suggestion, suggestTeam, loading, error }
}

export const useAIPrediction = () => {
  const [prediction, setPrediction] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const predictSuccess = async (projectId) => {
    setLoading(true)
    setError(null)
    try {
      const data = await aiAPI.predictSuccess(projectId)
      setPrediction(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to predict success')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { prediction, predictSuccess, loading, error }
}

export const useAIPricing = () => {
  const [pricing, setPricing] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const suggestPricing = async (projectData) => {
    setLoading(true)
    setError(null)
    try {
      const data = await aiAPI.suggestPricing(projectData)
      setPricing(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to suggest pricing')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { pricing, suggestPricing, loading, error }
}

export const useAIProposal = () => {
  const [proposal, setProposal] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const generateProposal = async (jobId, freelancerProfile) => {
    setLoading(true)
    setError(null)
    try {
      const data = await aiAPI.generateProposal(jobId, freelancerProfile)
      setProposal(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to generate proposal')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { proposal, generateProposal, loading, error }
}
