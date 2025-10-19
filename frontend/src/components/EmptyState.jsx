
import './EmptyState.css'

function EmptyState({ 
  icon = '📭',
  title,
  message,
  action,
  actionText = 'Get Started'
}) {
  return (
    <div className="empty-state">
      <div className="empty-icon">{icon}</div>
      <h3>{title}</h3>
      <p>{message}</p>
      {action && (
        <button onClick={action} className="empty-action-btn">
          {actionText}
        </button>
      )}
    </div>
  )
}

export default EmptyState
