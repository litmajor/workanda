import { useState } from 'react'
import './Messages.css'

function Messages() {
  const [selectedChat, setSelectedChat] = useState(1)
  const [messageText, setMessageText] = useState('')

  const chats = [
    {
      id: 1,
      name: 'Sarah Chen',
      lastMessage: 'Sounds great! When can you start?',
      time: '2 min ago',
      unread: 2,
      avatar: 'S',
      online: true
    },
    {
      id: 2,
      name: 'Mike Rodriguez',
      lastMessage: 'I have sent the files for review',
      time: '1 hour ago',
      unread: 0,
      avatar: 'M',
      online: true
    },
    {
      id: 3,
      name: 'Emily Watson',
      lastMessage: 'Thank you for the quick response!',
      time: '3 hours ago',
      unread: 0,
      avatar: 'E',
      online: false
    },
    {
      id: 4,
      name: 'David Kim',
      lastMessage: 'Can we schedule a call to discuss?',
      time: '1 day ago',
      unread: 1,
      avatar: 'D',
      online: false
    }
  ]

  const messages = {
    1: [
      { id: 1, sender: 'them', text: 'Hi! I saw your proposal and I'm impressed with your portfolio.', time: '10:30 AM' },
      { id: 2, sender: 'me', text: 'Thank you! I'm excited about this project. I have experience with similar e-commerce platforms.', time: '10:32 AM' },
      { id: 3, sender: 'them', text: 'That's perfect! What's your availability like?', time: '10:35 AM' },
      { id: 4, sender: 'me', text: 'I can start next week. I estimate 8-10 weeks for completion based on the requirements.', time: '10:37 AM' },
      { id: 5, sender: 'them', text: 'Sounds great! When can you start?', time: '10:40 AM' }
    ],
    2: [
      { id: 1, sender: 'them', text: 'Hey, how's the project going?', time: 'Yesterday' },
      { id: 2, sender: 'me', text: 'Going well! I've completed the authentication module.', time: 'Yesterday' },
      { id: 3, sender: 'them', text: 'I've sent the files for review', time: 'Today 9:00 AM' }
    ]
  }

  const currentMessages = messages[selectedChat] || []

  const handleSendMessage = (e) => {
    e.preventDefault()
    if (messageText.trim()) {
      console.log('Sending message:', messageText)
      setMessageText('')
    }
  }

  return (
    <div className="messages-page">
      <div className="messages-container">
        <div className="chats-sidebar">
          <div className="chats-header">
            <h2>Messages</h2>
            <button className="btn btn-primary btn-small">New Message</button>
          </div>
          <div className="search-box">
            <input type="text" placeholder="Search conversations..." />
          </div>
          <div className="chats-list">
            {chats.map(chat => (
              <div
                key={chat.id}
                className={`chat-item ${selectedChat === chat.id ? 'active' : ''}`}
                onClick={() => setSelectedChat(chat.id)}
              >
                <div className="chat-avatar">
                  {chat.avatar}
                  {chat.online && <span className="online-indicator"></span>}
                </div>
                <div className="chat-info">
                  <div className="chat-header-row">
                    <h4>{chat.name}</h4>
                    <span className="chat-time">{chat.time}</span>
                  </div>
                  <div className="chat-last-message">
                    <p>{chat.lastMessage}</p>
                    {chat.unread > 0 && (
                      <span className="unread-badge">{chat.unread}</span>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="chat-main">
          <div className="chat-header">
            <div className="chat-header-info">
              <div className="chat-avatar">
                {chats.find(c => c.id === selectedChat)?.avatar}
              </div>
              <div>
                <h3>{chats.find(c => c.id === selectedChat)?.name}</h3>
                <span className="status">
                  {chats.find(c => c.id === selectedChat)?.online ? 'Online' : 'Offline'}
                </span>
              </div>
            </div>
            <div className="chat-actions">
              <button className="icon-btn">📞</button>
              <button className="icon-btn">📹</button>
              <button className="icon-btn">⋮</button>
            </div>
          </div>

          <div className="messages-area">
            {currentMessages.map(message => (
              <div key={message.id} className={`message ${message.sender}`}>
                <div className="message-content">
                  <p>{message.text}</p>
                  <span className="message-time">{message.time}</span>
                </div>
              </div>
            ))}
          </div>

          <form className="message-input-area" onSubmit={handleSendMessage}>
            <button type="button" className="icon-btn">📎</button>
            <input
              type="text"
              placeholder="Type a message..."
              value={messageText}
              onChange={(e) => setMessageText(e.target.value)}
            />
            <button type="button" className="icon-btn">😊</button>
            <button type="submit" className="btn btn-primary">Send</button>
          </form>
        </div>
      </div>
    </div>
  )
}

export default Messages
