import { useState, useEffect } from 'react';
import api from '../services/api';
import './EscrowManagement.css';

function EscrowManagement() {
  const [contracts, setContracts] = useState([]);
  const [selectedContract, setSelectedContract] = useState(null);
  const [milestones, setMilestones] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadContracts();
  }, []);

  const loadContracts = async () => {
    try {
      setLoading(true);
      const clientId = 1;
      const data = await api.contracts.getForClient(clientId);
      setContracts(data || []);
    } catch (error) {
      console.error('Failed to load contracts:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadMilestones = async (contractId) => {
    try {
      const data = await api.milestones.get(contractId);
      setMilestones(data || []);
    } catch (error) {
      console.error('Failed to load milestones:', error);
    }
  };

  const handleSelectContract = (contract) => {
    setSelectedContract(contract);
    loadMilestones(contract.id);
  };

  const handleReleaseFunds = async (milestone) => {
    if (!confirm('Are you sure you want to release funds for this milestone?')) return;

    try {
      const escrowId = milestone.escrow_id || selectedContract.escrow_id;
      if (!escrowId) {
        alert('No escrow account found for this milestone');
        return;
      }
      await api.escrow.release(escrowId);
      alert('Funds released successfully!');
      loadMilestones(selectedContract.id);
    } catch (error) {
      console.error('Failed to release funds:', error);
      alert('Failed to release funds');
    }
  };

  const handleRaiseDispute = async (milestone) => {
    const reason = prompt('Please describe the reason for the dispute:');
    if (!reason) return;

    try {
      const escrowId = milestone.escrow_id || selectedContract.escrow_id;
      if (!escrowId) {
        alert('No escrow account found for this milestone');
        return;
      }
      await api.escrow.handleDispute(escrowId, { reason });
      alert('Dispute raised successfully!');
    } catch (error) {
      console.error('Failed to raise dispute:', error);
      alert('Failed to raise dispute');
    }
  };

  return (
    <div className="escrow-management-page">
      <div className="page-header">
        <h1>💰 Escrow & Payment Management</h1>
        <p>Secure milestone-based payments for all your projects</p>
      </div>

      <div className="escrow-container">
        <div className="contracts-sidebar">
          <h3>Active Contracts</h3>
          {contracts.length === 0 ? (
            <p className="empty-message">No active contracts</p>
          ) : (
            contracts.map((contract) => (
              <div
                key={contract.id}
                className={`contract-item ${selectedContract?.id === contract.id ? 'active' : ''}`}
                onClick={() => handleSelectContract(contract)}
              >
                <h4>Project #{contract.project_id}</h4>
                <p className="contract-status">{contract.status}</p>
                <p className="contract-amount">${contract.total_amount?.toLocaleString() || 'N/A'}</p>
              </div>
            ))
          )}
        </div>

        <div className="milestone-details">
          {selectedContract ? (
            <>
              <div className="contract-header">
                <h2>Contract #{selectedContract.id}</h2>
                <div className="contract-meta">
                  <span className="status-badge">{selectedContract.status}</span>
                  <span className="amount">${selectedContract.total_amount?.toLocaleString()}</span>
                </div>
              </div>

              <div className="milestones-section">
                <h3>Payment Milestones</h3>

                {milestones.length === 0 ? (
                  <p className="empty-message">No milestones defined</p>
                ) : (
                  <div className="milestones-list">
                    {milestones.map((milestone) => (
                      <div key={milestone.id} className={`milestone-card ${milestone.status}`}>
                        <div className="milestone-header">
                          <h4>{milestone.title}</h4>
                          <span className={`status-badge ${milestone.status}`}>
                            {milestone.status}
                          </span>
                        </div>

                        <p className="milestone-description">{milestone.description}</p>

                        <div className="milestone-meta">
                          <div className="meta-item">
                            <span className="label">Amount:</span>
                            <span className="value">${milestone.payment_amount?.toLocaleString()}</span>
                          </div>
                          <div className="meta-item">
                            <span className="label">Due Date:</span>
                            <span className="value">
                              {milestone.due_date ? new Date(milestone.due_date).toLocaleDateString() : 'TBD'}
                            </span>
                          </div>
                        </div>

                        <div className="milestone-actions">
                          {milestone.status === 'completed' && (
                            <button
                              className="btn btn-primary"
                              onClick={() => handleReleaseFunds(milestone)}
                            >
                              Release Payment
                            </button>
                          )}
                          {milestone.status === 'in_progress' && (
                            <button
                              className="btn btn-secondary"
                              onClick={() => handleRaiseDispute(milestone)}
                            >
                              Raise Dispute
                            </button>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>

              <div className="escrow-protection-info">
                <h4>🔒 Escrow Protection</h4>
                <p>
                  All payments are held securely in escrow until milestones are completed and approved.
                  This protects both clients and freelancers, ensuring fair payment for quality work.
                </p>
                <ul>
                  <li>Funds released only after milestone completion</li>
                  <li>Dispute resolution available for any issues</li>
                  <li>Transparent payment history</li>
                  <li>Platform fee calculated automatically</li>
                </ul>
              </div>
            </>
          ) : (
            <div className="empty-state">
              <p>Select a contract to view milestones and manage payments</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default EscrowManagement;
