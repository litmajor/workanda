const API_BASE = '/api/v1';

class APIService {
  async request(endpoint, options = {}) {
    const url = `${API_BASE}${endpoint}`;
    const config = {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      ...options,
    };

    if (config.body && typeof config.body === 'object') {
      config.body = JSON.stringify(config.body);
    }

    try {
      const response = await fetch(url, config);
      
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: response.statusText }));
        throw new Error(error.message || 'Request failed');
      }

      return await response.json();
    } catch (error) {
      console.error(`API Error [${endpoint}]:`, error);
      throw error;
    }
  }

  get(endpoint, options) {
    return this.request(endpoint, { ...options, method: 'GET' });
  }

  post(endpoint, data, options) {
    return this.request(endpoint, { ...options, method: 'POST', body: data });
  }

  put(endpoint, data, options) {
    return this.request(endpoint, { ...options, method: 'PUT', body: data });
  }

  delete(endpoint, options) {
    return this.request(endpoint, { ...options, method: 'DELETE' });
  }

  auth = {
    login: (credentials) => this.post('/auth/login', credentials),
    register: (userData) => this.post('/auth/register', userData),
    logout: () => this.post('/auth/logout'),
  };

  freelancer = {
    getProfile: () => this.get('/freelancer/profile'),
    updateProfile: (data) => this.put('/freelancer/update-profile', data),
    getJobs: () => this.get('/freelancer/jobs'),
    submitProposal: (jobId, proposal) => this.post(`/freelancer/submit-proposal/${jobId}`, proposal),
  };

  client = {
    getProfile: () => this.get('/client/profile'),
    updateProfile: (data) => this.put('/client/update-profile', data),
    createJob: (jobData) => this.post('/client/create-job', jobData),
    getJobs: () => this.get('/client/jobs'),
  };

  jobs = {
    browse: () => this.get('/jobs'),
    getDetails: (id) => this.get(`/jobs/${id}`),
  };

  proposals = {
    submit: (jobId, data) => this.post(`/proposals/${jobId}`, data),
    getForJob: (jobId) => this.get(`/proposals/${jobId}`),
    select: (proposalId) => this.post(`/proposals/${proposalId}/select`),
    update: (proposalId, data) => this.put(`/proposals/${proposalId}`, data),
    delete: (proposalId) => this.delete(`/proposals/${proposalId}`),
  };

  projects = {
    create: (data) => this.post('/projects', data),
    list: () => this.get('/projects'),
    get: (id) => this.get(`/projects/${id}`),
    update: (id, data) => this.put(`/projects/${id}`, data),
    delete: (id) => this.delete(`/projects/${id}`),
    addFreelancer: (projectId, freelancerId) => this.post(`/projects/${projectId}/add-freelancer/${freelancerId}`),
    removeFreelancer: (projectId, freelancerId) => this.delete(`/projects/${projectId}/remove-freelancer/${freelancerId}`),
  };

  contracts = {
    create: (data) => this.post('/contracts', data),
    getForClient: (clientId) => this.get(`/contracts/client/${clientId}`),
  };

  milestones = {
    create: (contractId, data) => this.post(`/milestones/${contractId}`, data),
    get: (contractId) => this.get(`/milestones/${contractId}`),
    update: (contractId, milestoneId, data) => this.put(`/milestones/${contractId}/${milestoneId}`, data),
    complete: (contractId, milestoneId) => this.post(`/milestones/${contractId}/${milestoneId}/complete`),
  };

  escrow = {
    create: (contractId) => this.post(`/escrow/${contractId}`),
    release: (escrowId) => this.post(`/escrow/${escrowId}/release`),
    refund: (escrowId) => this.post(`/escrow/${escrowId}/refund`),
    handleDispute: (escrowId, data) => this.post(`/dispute/${escrowId}`, data),
    escalate: (disputeId) => this.post(`/escalate/${disputeId}`),
  };

  payments = {
    create: (contractId, data) => this.post(`/payments/${contractId}`, data),
    get: (contractId) => this.get(`/payments/${contractId}`),
    updateStatus: (paymentId, data) => this.put(`/payments/${paymentId}`, data),
  };

  reviews = {
    add: (data) => this.post('/reviews', data),
    getForFreelancer: (freelancerId) => this.get(`/reviews/${freelancerId}`),
    getPaginated: (freelancerId, page) => this.get(`/reviews/${freelancerId}/paginated?page=${page}`),
    getAggregate: (freelancerId) => this.get(`/reviews/aggregate/${freelancerId}`),
    update: (reviewId, data) => this.put(`/reviews/${reviewId}`, data),
    delete: (reviewId) => this.delete(`/reviews/${reviewId}`),
  };

  messages = {
    send: (data) => this.post('/messages', data),
    getForUser: (userId) => this.get(`/messages/${userId}`),
  };

  teams = {
    create: (data) => this.post('/teams', data),
    get: (teamId) => this.get(`/teams/${teamId}`),
    update: (teamId, data) => this.put(`/teams/${teamId}`, data),
    submitProposal: (data) => this.post('/proposals/team', data),
    getProposals: (jobId) => this.get(`/proposals/team/job/${jobId}`),
  };

  agencies = {
    create: (data) => this.post('/agencies', data),
    get: (agencyId) => this.get(`/agencies/${agencyId}`),
    update: (agencyId, data) => this.put(`/agencies/${agencyId}`, data),
    addTeam: (agencyId, teamId) => this.post(`/agencies/${agencyId}/teams/${teamId}`),
    removeTeam: (agencyId, teamId) => this.delete(`/agencies/${agencyId}/teams/${teamId}`),
    getTeams: (agencyId) => this.get(`/agencies/${agencyId}/teams`),
  };

  ai = {
    getFreelancerMatches: (id) => this.get(`/ai/matches/freelancer/${id}`),
    getProjectMatches: (id) => this.get(`/ai/matches/project/${id}`),
    explainMatch: (data) => this.post('/ai/matches/explain', data),
    suggestTeam: (data) => this.post('/ai/team/suggest', data),
    formDynamicTeam: (data) => this.post('/ai/team/dynamic', data),
    analyzeSkillSynergy: (data) => this.post('/ai/team/synergy', data),
    analyzeProposal: (data) => this.post('/ai/proposal/analyze', data),
    categorizeJob: (id) => this.get(`/ai/job/categorize/${id}`),
    smartSearch: (query) => this.post('/ai/search', query),
  };

  predictive = {
    predictSuccess: (data) => this.post('/predictive/success', data),
    suggestPricing: (data) => this.post('/predictive/pricing', data),
    estimateTimeline: (data) => this.post('/predictive/timeline', data),
  };

  trust = {
    getScore: (userId) => this.get(`/trust/score/${userId}`),
    checkFraud: (userId) => this.get(`/trust/fraud/check/${userId}`),
    predictDisputeRisk: (contractId) => this.get(`/trust/dispute/risk/${contractId}`),
  };

  timeTracking = {
    create: (data) => this.post('/time-tracking/create', data),
    getForUser: (userId) => this.get(`/time-tracking/${userId}`),
    getTimesheet: (userId) => this.get(`/time-tracking/${userId}/timesheet`),
    getReport: (userId) => this.get(`/time-tracking/${userId}/report`),
  };
}

export default new APIService();
