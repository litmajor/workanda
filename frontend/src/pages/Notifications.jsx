
import { useState } from 'react'
import './Notifications.css'

function Notifications() {
  const [filter, setFilter] = useState('all')

  const notifications = [
    {
      id: 1,
      type: 'project',
      icon: '💼',
      title: 'New Project Invitation',
      message: 'Sarah Chen invited you to join "E-commerce Platform Development"',
      time: '5 minutes ago',
      read: false
    },
    {
      id: 2,
      type: 'payment',
      icon: '💰',
      title: 'Payment Received',
      message: 'You received $1,500 for milestone completion',
      time: '1 hour ago',
      read: false
    },
    {
      id: 3,
      type: 'message',
      icon: '💬',
      title: 'New Message',
      message: 'Mike Rodriguez sent you a message',
      time: '2 hours ago',
      read: true
    },
    {
      id: 4,
      type: 'review',
      icon: '⭐',
      title: 'New Review',
      message: 'Emily Watson left you a 5-star review',
      time: '1 day ago',
      read: true
    },
    {
      id: 5,
      type: 'proposal',
      icon: '📝',
      title: 'Proposal Update',
      message: 'Your proposal for "React Developer" was accepted',
      time: '2 days ago',
      read: true
    }
  ]

  const filteredNotifications = filter === 'all' 
    ? notifications 
    : filter === 'unread' 
    ? notifications.filter(n => !n.read)
    : notifications.filter(n => n.type === filter)

  const markAsRead = (id) => {
    console.log('Marking notification as read:', id)
  }

  const markAllAsRead = () => {
    console.log('Marking all as read')
  }

  return (
    <div className="notifications-page">
      <div className="notifications-container">
        <div className="notifications-header">
          <div>
            <h1>Notifications</h1>
            <p>{notifications.filter(n => !n.read).length} unread notifications</p>
          </div>
          <button className="btn btn-secondary" onClick={markAllAsRead}>
            Mark All as Read
          </button>
        </div>

        <div className="notifications-filters">
          <button
            className={`filter-btn ${filter === 'all' ? 'active' : ''}`}
            onClick={() => setFilter('all')}
          >
            All
          </button>
          <button
            className={`filter-btn ${filter === 'unread' ? 'active' : ''}`}
            onClick={() => setFilter('unread')}
          >
            Unread
          </button>
          <button
            className={`filter-btn ${filter === 'project' ? 'active' : ''}`}
            onClick={() => setFilter('project')}
          >
            Projects
          </button>
          <button
            className={`filter-btn ${filter === 'payment' ? 'active' : ''}`}
            onClick={() => setFilter('payment')}
          >
            Payments
          </button>
          <button
            className={`filter-btn ${filter === 'message' ? 'active' : ''}`}
            onClick={() => setFilter('message')}
          >
            Messages
          </button>
        </div>

        <div className="notifications-list">
          {filteredNotifications.map(notification => (
            <div
              key={notification.id}
              className={`notification-item ${notification.read ? 'read' : 'unread'}`}
              onClick={() => markAsRead(notification.id)}
            >
              <div className="notification-icon">{notification.icon}</div>
              <div className="notification-content">
                <h4>{notification.title}</h4>
                <p>{notification.message}</p>
                <span className="notification-time">{notification.time}</span>
              </div>
              {!notification.read && <div className="unread-indicator"></div>}
            </div>
          ))}
        </div>

        {filteredNotifications.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">🔔</div>
            <h3>No notifications</h3>
            <p>You're all caught up!</p>
          </div>
        )}
      </div>
    </div>
  )
}

export default Notifications
