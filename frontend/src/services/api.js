
const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api/v1'

// Helper function for making API requests
async function request(endpoint, options = {}) {
  const token = localStorage.getItem('authToken')
  
  const config = {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...(token && { Authorization: `Bearer ${token}` }),
      ...options.headers,
    },
  }

  try {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, config)
    
    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: 'Request failed' }))
      throw new Error(error.message || `HTTP ${response.status}`)
    }
    
    return await response.json()
  } catch (error) {
    console.error('API Error:', error)
    throw error
  }
}

// Auth API
export const authAPI = {
  login: async (email, password) => {
    const response = await request('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    })
    if (response.token) {
      localStorage.setItem('authToken', response.token)
      localStorage.setItem('user', JSON.stringify(response.user))
    }
    return response
  },

  signup: async (userData) => {
    const response = await request('/auth/register', {
      method: 'POST',
      body: JSON.stringify(userData),
    })
    if (response.token) {
      localStorage.setItem('authToken', response.token)
      localStorage.setItem('user', JSON.stringify(response.user))
    }
    return response
  },

  logout: () => {
    localStorage.removeItem('authToken')
    localStorage.removeItem('user')
  },

  getCurrentUser: () => {
    const user = localStorage.getItem('user')
    return user ? JSON.parse(user) : null
  },

  verifyMFA: async (code) => {
    return await request('/auth/mfa/verify', {
      method: 'POST',
      body: JSON.stringify({ code }),
    })
  },
}

// Jobs API
export const jobsAPI = {
  getAll: async (filters = {}) => {
    const params = new URLSearchParams(filters).toString()
    return await request(`/jobs?${params}`)
  },

  getById: async (id) => {
    return await request(`/jobs/${id}`)
  },

  create: async (jobData) => {
    return await request('/jobs', {
      method: 'POST',
      body: JSON.stringify(jobData),
    })
  },

  update: async (id, jobData) => {
    return await request(`/jobs/${id}`, {
      method: 'PUT',
      body: JSON.stringify(jobData),
    })
  },

  delete: async (id) => {
    return await request(`/jobs/${id}`, {
      method: 'DELETE',
    })
  },

  search: async (query) => {
    return await request(`/search/jobs?q=${encodeURIComponent(query)}`)
  },
}

// Projects API
export const projectsAPI = {
  getAll: async () => {
    return await request('/projects')
  },

  getById: async (id) => {
    return await request(`/projects/${id}`)
  },

  create: async (projectData) => {
    return await request('/projects', {
      method: 'POST',
      body: JSON.stringify(projectData),
    })
  },

  update: async (id, projectData) => {
    return await request(`/projects/${id}`, {
      method: 'PUT',
      body: JSON.stringify(projectData),
    })
  },

  updateStatus: async (id, status) => {
    return await request(`/projects/${id}/status`, {
      method: 'PATCH',
      body: JSON.stringify({ status }),
    })
  },

  getTasks: async (projectId) => {
    return await request(`/projects/${projectId}/tasks`)
  },

  createTask: async (projectId, taskData) => {
    return await request(`/projects/${projectId}/tasks`, {
      method: 'POST',
      body: JSON.stringify(taskData),
    })
  },
}

// Messages API
export const messagesAPI = {
  getConversations: async () => {
    return await request('/messages/conversations')
  },

  getMessages: async (userId) => {
    return await request(`/messages/${userId}`)
  },

  sendMessage: async (userId, content, attachments = []) => {
    return await request('/messages', {
      method: 'POST',
      body: JSON.stringify({ recipient_id: userId, content, attachments }),
    })
  },

  markAsRead: async (messageId) => {
    return await request(`/messages/${messageId}/read`, {
      method: 'PATCH',
    })
  },
}

// Payments API
export const paymentsAPI = {
  createEscrow: async (projectId, amount) => {
    return await request('/escrow', {
      method: 'POST',
      body: JSON.stringify({ project_id: projectId, amount }),
    })
  },

  releaseFunds: async (escrowId, milestoneId) => {
    return await request(`/escrow/${escrowId}/release`, {
      method: 'POST',
      body: JSON.stringify({ milestone_id: milestoneId }),
    })
  },

  getPaymentHistory: async () => {
    return await request('/payments/history')
  },

  createMilestone: async (projectId, milestoneData) => {
    return await request(`/projects/${projectId}/milestones`, {
      method: 'POST',
      body: JSON.stringify(milestoneData),
    })
  },

  approveMilestone: async (milestoneId) => {
    return await request(`/milestones/${milestoneId}/approve`, {
      method: 'POST',
    })
  },
}

// AI API
export const aiAPI = {
  getJobMatches: async (jobId) => {
    return await request(`/ai/matching/jobs/${jobId}`)
  },

  getFreelancerMatches: async (freelancerId) => {
    return await request(`/ai/matching/freelancers/${freelancerId}`)
  },

  suggestTeam: async (projectRequirements) => {
    return await request('/ai/team-formation', {
      method: 'POST',
      body: JSON.stringify(projectRequirements),
    })
  },

  predictSuccess: async (projectId) => {
    return await request(`/ai/predictive/success/${projectId}`)
  },

  suggestPricing: async (projectData) => {
    return await request('/ai/pricing', {
      method: 'POST',
      body: JSON.stringify(projectData),
    })
  },

  generateProposal: async (jobId, freelancerProfile) => {
    return await request('/ai/proposal-assistant', {
      method: 'POST',
      body: JSON.stringify({ job_id: jobId, profile: freelancerProfile }),
    })
  },
}

// Proposals API
export const proposalsAPI = {
  create: async (proposalData) => {
    return await request('/proposals', {
      method: 'POST',
      body: JSON.stringify(proposalData),
    })
  },

  getByJob: async (jobId) => {
    return await request(`/jobs/${jobId}/proposals`)
  },

  getByFreelancer: async (freelancerId) => {
    return await request(`/freelancers/${freelancerId}/proposals`)
  },

  accept: async (proposalId) => {
    return await request(`/proposals/${proposalId}/accept`, {
      method: 'POST',
    })
  },

  reject: async (proposalId) => {
    return await request(`/proposals/${proposalId}/reject`, {
      method: 'POST',
    })
  },
}

// Teams API
export const teamsAPI = {
  getAll: async () => {
    return await request('/teams')
  },

  create: async (teamData) => {
    return await request('/teams', {
      method: 'POST',
      body: JSON.stringify(teamData),
    })
  },

  addMember: async (teamId, userId, role) => {
    return await request(`/teams/${teamId}/members`, {
      method: 'POST',
      body: JSON.stringify({ user_id: userId, role }),
    })
  },

  removeMember: async (teamId, userId) => {
    return await request(`/teams/${teamId}/members/${userId}`, {
      method: 'DELETE',
    })
  },
}

// WebSocket connection
export class WebSocketService {
  constructor() {
    this.ws = null
    this.listeners = new Map()
  }

  connect() {
    const token = localStorage.getItem('authToken')
    const wsUrl = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws'
    
    this.ws = new WebSocket(`${wsUrl}?token=${token}`)
    
    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data)
      const listeners = this.listeners.get(data.type) || []
      listeners.forEach(callback => callback(data))
    }

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    this.ws.onclose = () => {
      console.log('WebSocket disconnected, reconnecting...')
      setTimeout(() => this.connect(), 3000)
    }
  }

  on(eventType, callback) {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, [])
    }
    this.listeners.get(eventType).push(callback)
  }

  off(eventType, callback) {
    const listeners = this.listeners.get(eventType) || []
    this.listeners.set(eventType, listeners.filter(cb => cb !== callback))
  }

  send(type, data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type, ...data }))
    }
  }

  disconnect() {
    if (this.ws) {
      this.ws.close()
    }
  }
}

export const wsService = new WebSocketService()
