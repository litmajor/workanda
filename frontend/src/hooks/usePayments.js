
import { useState, useEffect } from 'react'
import { paymentsAPI } from '../services/api'
import { useApp } from '../context/AppContext'

export const usePaymentHistory = () => {
  const [payments, setPayments] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchPayments = async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await paymentsAPI.getPaymentHistory()
      setPayments(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch payment history')
      throw err
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchPayments()
  }, [])

  return { payments, loading, error, refetch: fetchPayments }
}

export const useEscrow = () => {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const createEscrow = async (projectId, amount) => {
    setLoading(true)
    setError(null)
    try {
      const data = await paymentsAPI.createEscrow(projectId, amount)
      addAlert('success', 'Escrow created successfully!')
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to create escrow')
      throw err
    } finally {
      setLoading(false)
    }
  }

  const releaseFunds = async (escrowId, milestoneId) => {
    setLoading(true)
    setError(null)
    try {
      const data = await paymentsAPI.releaseFunds(escrowId, milestoneId)
      addAlert('success', 'Funds released successfully!')
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to release funds')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { createEscrow, releaseFunds, loading, error }
}

export const useMilestones = (projectId) => {
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const createMilestone = async (milestoneData) => {
    setLoading(true)
    setError(null)
    try {
      const data = await paymentsAPI.createMilestone(projectId, milestoneData)
      addAlert('success', 'Milestone created successfully!')
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to create milestone')
      throw err
    } finally {
      setLoading(false)
    }
  }

  const approveMilestone = async (milestoneId) => {
    setLoading(true)
    setError(null)
    try {
      const data = await paymentsAPI.approveMilestone(milestoneId)
      addAlert('success', 'Milestone approved!')
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to approve milestone')
      throw err
    } finally {
      setLoading(false)
    }
  }

  return { createMilestone, approveMilestone, loading, error }
}
