import { useState, useEffect, useRef } from 'react';
import api from '../services/api';
import Loader from '../components/Loader';
import './TimeTracking.css';

function TimeTracking() {
  const [activeTimer, setActiveTimer] = useState(null);
  const [timeEntries, setTimeEntries] = useState([]);
  const [loading, setLoading] = useState(true);
  const [startTime, setStartTime] = useState(null);
  const [elapsedTime, setElapsedTime] = useState(0);
  const timerIntervalRef = useRef(null);

  useEffect(() => {
    loadTimeEntries();
    
    return () => {
      if (timerIntervalRef.current) {
        clearInterval(timerIntervalRef.current);
      }
    };
  }, []);

  const loadTimeEntries = async () => {
    try {
      setLoading(true);
      const userId = 1;
      const data = await api.timeTracking.getForUser(userId);
      setTimeEntries(data || []);
    } catch (error) {
      console.error('Failed to load time entries:', error);
    } finally {
      setLoading(false);
    }
  };

  const startTimer = (project) => {
    setActiveTimer(project);
    const now = Date.now();
    setStartTime(now);
    setElapsedTime(0);

    timerIntervalRef.current = setInterval(() => {
      setElapsedTime((prev) => prev + 1);
    }, 1000);
  };

  const stopTimer = async () => {
    if (timerIntervalRef.current) {
      clearInterval(timerIntervalRef.current);
      timerIntervalRef.current = null;
    }

    const hours = elapsedTime / 3600;
    
    try {
      const newEntry = {
        user_id: 1,
        project_id: 1,
        task_description: `Work on ${activeTimer}`,
        hours_worked: Number(hours.toFixed(2)),
        date: new Date().toISOString().split('T')[0],
        billable: true,
      };

      await api.timeTracking.create(newEntry);
      await loadTimeEntries();
      
      setActiveTimer(null);
      setStartTime(null);
      setElapsedTime(0);
    } catch (error) {
      console.error('Failed to save time entry:', error);
      alert('Failed to save time entry. Please try again.');
    }
  };

  const formatTime = (seconds) => {
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${String(hrs).padStart(2, '0')}:${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
  };

  const totalHours = timeEntries.reduce((sum, entry) => sum + (entry.hours_worked || 0), 0);
  const hourlyRate = 50;
  const totalEarnings = timeEntries
    .filter(e => e.billable)
    .reduce((sum, entry) => sum + ((entry.hours_worked || 0) * hourlyRate), 0);

  if (loading) {
    return <div className="time-tracking-page"><Loader /></div>;
  }

  return (
    <div className="time-tracking-page">
      <div className="page-header">
        <h1>⏱️ Time Tracking</h1>
        <p>Track your billable hours and maximize your earnings</p>
      </div>

      <div className="time-tracking-grid">
        <div className="timer-card">
          <h3>Active Timer</h3>

          {!activeTimer ? (
            <div className="start-timer-section">
              <p>Start tracking time for a project</p>
              <input
                type="text"
                placeholder="Project name..."
                className="project-input"
                onKeyPress={(e) => {
                  if (e.key === 'Enter' && e.target.value) {
                    startTimer(e.target.value);
                    e.target.value = '';
                  }
                }}
              />
              <button className="btn btn-primary" onClick={() => {
                const input = document.querySelector('.project-input');
                if (input && input.value) {
                  startTimer(input.value);
                  input.value = '';
                }
              }}>
                Start Timer
              </button>
            </div>
          ) : (
            <div className="active-timer-display">
              <div className="timer-display">{formatTime(elapsedTime)}</div>
              <p className="timer-project">{activeTimer}</p>
              <button className="btn btn-danger" onClick={stopTimer}>
                Stop Timer
              </button>
            </div>
          )}
        </div>

        <div className="stats-cards">
          <div className="stat-card">
            <h4>Total Hours (This Week)</h4>
            <div className="stat-value">{totalHours.toFixed(1)}h</div>
          </div>
          <div className="stat-card">
            <h4>Total Earnings</h4>
            <div className="stat-value">${totalEarnings.toFixed(2)}</div>
          </div>
        </div>
      </div>

      <div className="time-entries-section">
        <h3>Recent Time Entries</h3>

        {timeEntries.length === 0 ? (
          <p className="empty-message">No time entries yet. Start tracking to see your work history.</p>
        ) : (
          <table className="time-entries-table">
            <thead>
              <tr>
                <th>Date</th>
                <th>Task</th>
                <th>Hours</th>
                <th>Rate</th>
                <th>Amount</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {timeEntries.map((entry) => (
                <tr key={entry.id}>
                  <td>{entry.date ? new Date(entry.date).toLocaleDateString() : 'N/A'}</td>
                  <td className="project-cell">{entry.task_description || 'N/A'}</td>
                  <td>{entry.hours_worked?.toFixed(1) || 0}h</td>
                  <td>${hourlyRate}/hr</td>
                  <td className="amount-cell">${((entry.hours_worked || 0) * hourlyRate).toFixed(2)}</td>
                  <td>
                    <span className={`billable-badge ${entry.billable ? 'billable' : 'non-billable'}`}>
                      {entry.billable ? 'Billable' : 'Non-billable'}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export default TimeTracking;
