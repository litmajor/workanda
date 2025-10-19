
import './AccessibleButton.css'

function AccessibleButton({ 
  children,
  onClick,
  variant = 'primary',
  size = 'medium',
  disabled = false,
  loading = false,
  ariaLabel,
  className = '',
  ...props
}) {
  return (
    <button
      className={`accessible-btn btn-${variant} btn-${size} ${className}`}
      onClick={onClick}
      disabled={disabled || loading}
      aria-label={ariaLabel || (typeof children === 'string' ? children : undefined)}
      aria-busy={loading}
      {...props}
    >
      {loading ? (
        <span className="btn-loader" aria-hidden="true">
          <span className="spinner"></span>
        </span>
      ) : null}
      <span className={loading ? 'btn-content-hidden' : ''}>{children}</span>
    </button>
  )
}

export default AccessibleButton
