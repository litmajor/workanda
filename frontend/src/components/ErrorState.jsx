
import './ErrorState.css'

function ErrorState({ 
  type = 'error',
  title,
  message,
  action,
  actionText = 'Try Again'
}) {
  const getIcon = () => {
    switch (type) {
      case '404':
        return '🔍'
      case 'network':
        return '📡'
      case 'permission':
        return '🔒'
      case 'server':
        return '⚠️'
      default:
        return '❌'
    }
  }

  const getDefaultTitle = () => {
    switch (type) {
      case '404':
        return 'Page Not Found'
      case 'network':
        return 'Connection Error'
      case 'permission':
        return 'Access Denied'
      case 'server':
        return 'Server Error'
      default:
        return 'Something Went Wrong'
    }
  }

  const getDefaultMessage = () => {
    switch (type) {
      case '404':
        return "The page you're looking for doesn't exist."
      case 'network':
        return 'Please check your internet connection and try again.'
      case 'permission':
        return "You don't have permission to access this resource."
      case 'server':
        return 'Our servers are experiencing issues. Please try again later.'
      default:
        return 'An unexpected error occurred. Please try again.'
    }
  }

  return (
    <div className="error-state">
      <div className="error-icon">{getIcon()}</div>
      <h2>{title || getDefaultTitle()}</h2>
      <p>{message || getDefaultMessage()}</p>
      {action && (
        <button onClick={action} className="error-action-btn">
          {actionText}
        </button>
      )}
    </div>
  )
}

export default ErrorState
