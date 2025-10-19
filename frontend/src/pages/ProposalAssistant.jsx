import { useState } from 'react';
import api from '../services/api';
import './ProposalAssistant.css';

function ProposalAssistant() {
  const [jobId, setJobId] = useState('');
  const [draftProposal, setDraftProposal] = useState('');
  const [analysis, setAnalysis] = useState(null);
  const [loading, setLoading] = useState(false);

  const analyzeProposal = async () => {
    if (!jobId) {
      alert('Please enter a job ID');
      return;
    }

    try {
      setLoading(true);
      const data = await api.ai.analyzeProposal({
        job_id: parseInt(jobId),
        freelancer_id: 1,
        draft_proposal: draftProposal || undefined,
      });
      setAnalysis(data);
    } catch (error) {
      console.error('Failed to analyze proposal:', error);
      alert('Failed to analyze proposal. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const getScoreColor = (score) => {
    if (score >= 0.8) return '#10b981';
    if (score >= 0.6) return '#f59e0b';
    return '#ef4444';
  };

  return (
    <div className="proposal-assistant-page">
      <div className="page-header">
        <h1>✨ AI Proposal Assistant</h1>
        <p>Get AI-powered suggestions to write winning proposals</p>
      </div>

      <div className="assistant-container">
        <div className="input-section">
          <div className="form-group">
            <label>Job ID</label>
            <input
              type="number"
              value={jobId}
              onChange={(e) => setJobId(e.target.value)}
              placeholder="Enter the job ID you want to apply for"
              className="form-input"
            />
          </div>

          <div className="form-group">
            <label>Your Draft Proposal (Optional)</label>
            <textarea
              value={draftProposal}
              onChange={(e) => setDraftProposal(e.target.value)}
              placeholder="Paste your draft proposal here for AI analysis and improvements..."
              className="form-textarea"
              rows="10"
            />
          </div>

          <button
            className="btn btn-primary btn-large"
            onClick={analyzeProposal}
            disabled={loading}
          >
            {loading ? 'Analyzing...' : '🤖 Analyze with AI'}
          </button>
        </div>

        {analysis && (
          <div className="analysis-results">
            <div className="win-rate-card">
              <h3>Estimated Win Rate</h3>
              <div className="win-rate-score">
                {(analysis.estimated_win_rate * 100).toFixed(0)}%
              </div>
              <p>Probability of winning this project</p>
            </div>

            {analysis.improvements && (
              <div className="improvements-card">
                <h3>Proposal Quality Scores</h3>
                <div className="score-grid">
                  <div className="score-item">
                    <label>Structure</label>
                    <div className="score-bar">
                      <div
                        className="score-fill"
                        style={{
                          width: `${analysis.improvements.structure_score * 100}%`,
                          backgroundColor: getScoreColor(analysis.improvements.structure_score),
                        }}
                      />
                    </div>
                    <span>{(analysis.improvements.structure_score * 100).toFixed(0)}%</span>
                  </div>

                  <div className="score-item">
                    <label>Relevance</label>
                    <div className="score-bar">
                      <div
                        className="score-fill"
                        style={{
                          width: `${analysis.improvements.relevance_score * 100}%`,
                          backgroundColor: getScoreColor(analysis.improvements.relevance_score),
                        }}
                      />
                    </div>
                    <span>{(analysis.improvements.relevance_score * 100).toFixed(0)}%</span>
                  </div>

                  <div className="score-item">
                    <label>Clarity</label>
                    <div className="score-bar">
                      <div
                        className="score-fill"
                        style={{
                          width: `${analysis.improvements.clarity_score * 100}%`,
                          backgroundColor: getScoreColor(analysis.improvements.clarity_score),
                        }}
                      />
                    </div>
                    <span>{(analysis.improvements.clarity_score * 100).toFixed(0)}%</span>
                  </div>

                  <div className="score-item">
                    <label>Professionalism</label>
                    <div className="score-bar">
                      <div
                        className="score-fill"
                        style={{
                          width: `${analysis.improvements.professionalism_score * 100}%`,
                          backgroundColor: getScoreColor(analysis.improvements.professionalism_score),
                        }}
                      />
                    </div>
                    <span>{(analysis.improvements.professionalism_score * 100).toFixed(0)}%</span>
                  </div>
                </div>

                {analysis.improvements.strengths && analysis.improvements.strengths.length > 0 && (
                  <div className="feedback-section">
                    <h4>✅ Strengths</h4>
                    <ul>
                      {analysis.improvements.strengths.map((strength, idx) => (
                        <li key={idx} className="strength-item">{strength}</li>
                      ))}
                    </ul>
                  </div>
                )}

                {analysis.improvements.missing_elements && analysis.improvements.missing_elements.length > 0 && (
                  <div className="feedback-section">
                    <h4>⚠️ Areas for Improvement</h4>
                    <ul>
                      {analysis.improvements.missing_elements.map((element, idx) => (
                        <li key={idx} className="improvement-item">{element}</li>
                      ))}
                    </ul>
                  </div>
                )}
              </div>
            )}

            {analysis.assistant && (
              <div className="suggestions-card">
                <h3>AI Suggestions</h3>

                {analysis.assistant.suggested_structure && (
                  <div className="suggestion-section">
                    <h4>📝 Recommended Structure</h4>
                    <ol className="structure-list">
                      {analysis.assistant.suggested_structure.map((section, idx) => (
                        <li key={idx}>{section}</li>
                      ))}
                    </ol>
                  </div>
                )}

                {analysis.assistant.key_points && analysis.assistant.key_points.length > 0 && (
                  <div className="suggestion-section">
                    <h4>💡 Key Points to Include</h4>
                    <ul className="key-points-list">
                      {analysis.assistant.key_points.map((point, idx) => (
                        <li key={idx}>{point}</li>
                      ))}
                    </ul>
                  </div>
                )}

                {analysis.suggestions && analysis.suggestions.length > 0 && (
                  <div className="suggestion-section">
                    <h4>🎯 Personalized Tips</h4>
                    <div className="tips-grid">
                      {analysis.suggestions.map((suggestion, idx) => (
                        <div key={idx} className="tip-card">
                          {suggestion}
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

export default ProposalAssistant;
