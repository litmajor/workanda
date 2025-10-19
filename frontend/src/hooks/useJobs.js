
import { useState, useEffect } from 'react'
import { jobsAPI } from '../services/api'
import { useApp } from '../context/AppContext'

export const useJobs = (filters = {}) => {
  const [jobs, setJobs] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchJobs = async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await jobsAPI.getAll(filters)
      setJobs(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch jobs')
      throw err
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchJobs()
  }, [JSON.stringify(filters)])

  return { jobs, loading, error, refetch: fetchJobs }
}

export const useJob = (id) => {
  const [job, setJob] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchJob = async () => {
    if (!id) return
    setLoading(true)
    setError(null)
    try {
      const data = await jobsAPI.getById(id)
      setJob(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch job')
      throw err
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchJob()
  }, [id])

  return { job, loading, error, refetch: fetchJob }
}

export const useCreateJob = () => {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const createJob = async (jobData) => {
    setLoading(true)
    setError(null)
    try {
      const data = await jobsAPI.create(jobData)
      addAlert('success', 'Job posted successfully!')
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to create job')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { createJob, loading, error }
}

export const useSearchJobs = () => {
  const [results, setResults] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)

  const searchJobs = async (query) => {
    if (!query) {
      setResults([])
      return
    }
    setLoading(true)
    setError(null)
    try {
      const data = await jobsAPI.search(query)
      setResults(data)
      return data
    } catch (err) {
      setError(err.message)
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { results, searchJobs, loading, error }
}
