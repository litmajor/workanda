
import { useState } from 'react'
import './Settings.css'

function Settings() {
  const [activeTab, setActiveTab] = useState('profile')
  const [settings, setSettings] = useState({
    email: 'user@example.com',
    notifications: {
      email: true,
      push: true,
      sms: false
    },
    privacy: {
      profileVisibility: 'public',
      showEmail: false,
      showPhone: false
    },
    preferences: {
      language: 'en',
      timezone: 'UTC',
      currency: 'USD'
    }
  })

  const handleToggle = (category, key) => {
    setSettings(prev => ({
      ...prev,
      [category]: {
        ...prev[category],
        [key]: !prev[category][key]
      }
    }))
  }

  const handleChange = (category, key, value) => {
    setSettings(prev => ({
      ...prev,
      [category]: {
        ...prev[category],
        [key]: value
      }
    }))
  }

  return (
    <div className="settings-page">
      <div className="settings-container">
        <div className="settings-header">
          <h1>Settings</h1>
          <p>Manage your account preferences and privacy</p>
        </div>

        <div className="settings-tabs">
          <button
            className={`tab ${activeTab === 'profile' ? 'active' : ''}`}
            onClick={() => setActiveTab('profile')}
          >
            Profile
          </button>
          <button
            className={`tab ${activeTab === 'notifications' ? 'active' : ''}`}
            onClick={() => setActiveTab('notifications')}
          >
            Notifications
          </button>
          <button
            className={`tab ${activeTab === 'privacy' ? 'active' : ''}`}
            onClick={() => setActiveTab('privacy')}
          >
            Privacy
          </button>
          <button
            className={`tab ${activeTab === 'preferences' ? 'active' : ''}`}
            onClick={() => setActiveTab('preferences')}
          >
            Preferences
          </button>
        </div>

        <div className="settings-content">
          {activeTab === 'profile' && (
            <div className="settings-section">
              <h2>Profile Settings</h2>
              <div className="form-group">
                <label>Email Address</label>
                <input type="email" value={settings.email} readOnly />
              </div>
              <div className="form-group">
                <label>Change Password</label>
                <button className="btn btn-secondary">Update Password</button>
              </div>
              <div className="form-group">
                <label>Two-Factor Authentication</label>
                <button className="btn btn-primary">Enable 2FA</button>
              </div>
            </div>
          )}

          {activeTab === 'notifications' && (
            <div className="settings-section">
              <h2>Notification Preferences</h2>
              <div className="setting-item">
                <div>
                  <h4>Email Notifications</h4>
                  <p>Receive updates via email</p>
                </div>
                <label className="toggle">
                  <input
                    type="checkbox"
                    checked={settings.notifications.email}
                    onChange={() => handleToggle('notifications', 'email')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <div className="setting-item">
                <div>
                  <h4>Push Notifications</h4>
                  <p>Receive browser notifications</p>
                </div>
                <label className="toggle">
                  <input
                    type="checkbox"
                    checked={settings.notifications.push}
                    onChange={() => handleToggle('notifications', 'push')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <div className="setting-item">
                <div>
                  <h4>SMS Notifications</h4>
                  <p>Receive text message alerts</p>
                </div>
                <label className="toggle">
                  <input
                    type="checkbox"
                    checked={settings.notifications.sms}
                    onChange={() => handleToggle('notifications', 'sms')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
            </div>
          )}

          {activeTab === 'privacy' && (
            <div className="settings-section">
              <h2>Privacy Settings</h2>
              <div className="form-group">
                <label>Profile Visibility</label>
                <select
                  value={settings.privacy.profileVisibility}
                  onChange={(e) => handleChange('privacy', 'profileVisibility', e.target.value)}
                >
                  <option value="public">Public</option>
                  <option value="private">Private</option>
                  <option value="connections">Connections Only</option>
                </select>
              </div>
              <div className="setting-item">
                <div>
                  <h4>Show Email</h4>
                  <p>Display email on your profile</p>
                </div>
                <label className="toggle">
                  <input
                    type="checkbox"
                    checked={settings.privacy.showEmail}
                    onChange={() => handleToggle('privacy', 'showEmail')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
              <div className="setting-item">
                <div>
                  <h4>Show Phone</h4>
                  <p>Display phone number on your profile</p>
                </div>
                <label className="toggle">
                  <input
                    type="checkbox"
                    checked={settings.privacy.showPhone}
                    onChange={() => handleToggle('privacy', 'showPhone')}
                  />
                  <span className="slider"></span>
                </label>
              </div>
            </div>
          )}

          {activeTab === 'preferences' && (
            <div className="settings-section">
              <h2>User Preferences</h2>
              <div className="form-group">
                <label>Language</label>
                <select
                  value={settings.preferences.language}
                  onChange={(e) => handleChange('preferences', 'language', e.target.value)}
                >
                  <option value="en">English</option>
                  <option value="es">Spanish</option>
                  <option value="fr">French</option>
                </select>
              </div>
              <div className="form-group">
                <label>Timezone</label>
                <select
                  value={settings.preferences.timezone}
                  onChange={(e) => handleChange('preferences', 'timezone', e.target.value)}
                >
                  <option value="UTC">UTC</option>
                  <option value="EST">Eastern Time</option>
                  <option value="PST">Pacific Time</option>
                </select>
              </div>
              <div className="form-group">
                <label>Currency</label>
                <select
                  value={settings.preferences.currency}
                  onChange={(e) => handleChange('preferences', 'currency', e.target.value)}
                >
                  <option value="USD">USD</option>
                  <option value="EUR">EUR</option>
                  <option value="GBP">GBP</option>
                </select>
              </div>
            </div>
          )}

          <div className="settings-actions">
            <button className="btn btn-primary">Save Changes</button>
            <button className="btn btn-secondary">Cancel</button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Settings
