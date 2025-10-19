
import { useState, useEffect } from 'react'
import { projectsAPI } from '../services/api'
import { useApp } from '../context/AppContext'

export const useProjects = () => {
  const [projects, setProjects] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchProjects = async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await projectsAPI.getAll()
      setProjects(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch projects')
      throw err
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchProjects()
  }, [])

  return { projects, loading, error, refetch: fetchProjects }
}

export const useProject = (id) => {
  const [project, setProject] = useState(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchProject = async () => {
    if (!id) return
    setLoading(true)
    setError(null)
    try {
      const data = await projectsAPI.getById(id)
      setProject(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch project')
      throw err
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchProject()
  }, [id])

  const updateStatus = async (status) => {
    try {
      const data = await projectsAPI.updateStatus(id, status)
      setProject(data)
      addAlert('success', 'Project status updated!')
      return data
    } catch (err) {
      addAlert('error', err.message || 'Failed to update status')
      throw err
    }
  }

  return { project, loading, error, refetch: fetchProject, updateStatus }
}

export const useProjectTasks = (projectId) => {
  const [tasks, setTasks] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchTasks = async () => {
    if (!projectId) return
    setLoading(true)
    setError(null)
    try {
      const data = await projectsAPI.getTasks(projectId)
      setTasks(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch tasks')
      throw err
    } finally {
      setLoading(false)
    }
  }

  const createTask = async (taskData) => {
    try {
      const data = await projectsAPI.createTask(projectId, taskData)
      setTasks([...tasks, data])
      addAlert('success', 'Task created successfully!')
      return data
    } catch (err) {
      addAlert('error', err.message || 'Failed to create task')
      throw err
    }
  }

  useEffect(() => {
    fetchTasks()
  }, [projectId])

  return { tasks, loading, error, refetch: fetchTasks, createTask }
}
