import { useState, useEffect } from 'react';
import api from '../services/api';
import Loader from '../components/Loader';
import './AIMatching.css';

function AIMatching() {
  const [loading, setLoading] = useState(true);
  const [matches, setMatches] = useState([]);
  const [selectedMatch, setSelectedMatch] = useState(null);
  const [explanation, setExplanation] = useState(null);

  useEffect(() => {
    loadMatches();
  }, []);

  const loadMatches = async () => {
    try {
      setLoading(true);
      const userId = 1;
      const data = await api.ai.getFreelancerMatches(userId);
      setMatches(data.matches || []);
    } catch (error) {
      console.error('Failed to load matches:', error);
    } finally {
      setLoading(false);
    }
  };

  const explainMatch = async (match) => {
    try {
      const data = await api.ai.explainMatch({
        freelancer_id: match.freelancer_id,
        project_id: match.project_id,
      });
      setExplanation(data);
      setSelectedMatch(match);
    } catch (error) {
      console.error('Failed to explain match:', error);
    }
  };

  const getScoreColor = (score) => {
    if (score >= 0.8) return '#10b981';
    if (score >= 0.6) return '#f59e0b';
    return '#ef4444';
  };

  const getScoreLabel = (score) => {
    if (score >= 0.8) return 'Excellent Match';
    if (score >= 0.6) return 'Good Match';
    return 'Fair Match';
  };

  if (loading) {
    return <div className="ai-matching-page"><Loader /></div>;
  }

  return (
    <div className="ai-matching-page">
      <div className="page-header">
        <h1>🤖 AI-Powered Project Matching</h1>
        <p>Find the perfect projects based on your skills, experience, and preferences</p>
      </div>

      <div className="matches-grid">
        {matches.length === 0 ? (
          <div className="empty-state">
            <p>No matches found. Complete your profile to get personalized recommendations.</p>
          </div>
        ) : (
          matches.map((match) => (
            <div key={match.id} className="match-card">
              <div className="match-header">
                <h3>{match.project_name || 'Project'}</h3>
                <div className="match-score" style={{ backgroundColor: getScoreColor(match.score) }}>
                  {(match.score * 100).toFixed(0)}%
                </div>
              </div>

              <div className="match-label">{getScoreLabel(match.score)}</div>

              <div className="match-details">
                <div className="detail">
                  <strong>Budget:</strong> ${match.budget?.toLocaleString() || 'N/A'}
                </div>
                <div className="detail">
                  <strong>Duration:</strong> {match.estimated_duration || 'TBD'}
                </div>
                <div className="detail">
                  <strong>Category:</strong> {match.category || 'General'}
                </div>
              </div>

              <div className="match-reasons">
                <h4>Why this match?</h4>
                <ul>
                  {(match.reasons || ['AI-powered compatibility analysis']).map((reason, idx) => (
                    <li key={idx}>{reason}</li>
                  ))}
                </ul>
              </div>

              <div className="match-actions">
                <button className="btn btn-secondary" onClick={() => explainMatch(match)}>
                  View Details
                </button>
                <button className="btn btn-primary">Apply Now</button>
              </div>
            </div>
          ))
        )}
      </div>

      {selectedMatch && explanation && (
        <div className="modal-overlay" onClick={() => setSelectedMatch(null)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>Match Analysis</h2>
              <button className="close-btn" onClick={() => setSelectedMatch(null)}>×</button>
            </div>
            <div className="modal-body">
              <div className="analysis-section">
                <h3>Skill Match</h3>
                <div className="progress-bar">
                  <div className="progress" style={{ width: `${(explanation.skill_score || 0) * 100}%` }}></div>
                </div>
                <p>{(explanation.skill_score * 100).toFixed(0)}% skill compatibility</p>
              </div>

              <div className="analysis-section">
                <h3>Experience Level</h3>
                <div className="progress-bar">
                  <div className="progress" style={{ width: `${(explanation.experience_score || 0) * 100}%` }}></div>
                </div>
                <p>{(explanation.experience_score * 100).toFixed(0)}% experience match</p>
              </div>

              <div className="analysis-section">
                <h3>Budget Fit</h3>
                <div className="progress-bar">
                  <div className="progress" style={{ width: `${(explanation.budget_score || 0) * 100}%` }}></div>
                </div>
                <p>{(explanation.budget_score * 100).toFixed(0)}% budget alignment</p>
              </div>

              <div className="analysis-section">
                <h3>Success Probability</h3>
                <div className="probability-score">
                  {((explanation.success_probability || 0) * 100).toFixed(0)}%
                </div>
                <p>Estimated likelihood of successful project completion</p>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default AIMatching;
