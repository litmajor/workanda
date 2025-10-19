
import './Input.css'

export function TextInput({ label, error, ...props }) {
  return (
    <div className="input-group">
      {label && <label className="input-label">{label}</label>}
      <input 
        type="text" 
        className={`input ${error ? 'input-error' : ''}`}
        {...props}
      />
      {error && <span className="input-error-text">{error}</span>}
    </div>
  )
}

export function TextArea({ label, error, rows = 4, ...props }) {
  return (
    <div className="input-group">
      {label && <label className="input-label">{label}</label>}
      <textarea 
        className={`input textarea ${error ? 'input-error' : ''}`}
        rows={rows}
        {...props}
      />
      {error && <span className="input-error-text">{error}</span>}
    </div>
  )
}

export function Select({ label, error, options, ...props }) {
  return (
    <div className="input-group">
      {label && <label className="input-label">{label}</label>}
      <select 
        className={`input ${error ? 'input-error' : ''}`}
        {...props}
      >
        {options.map(opt => (
          <option key={opt.value} value={opt.value}>
            {opt.label}
          </option>
        ))}
      </select>
      {error && <span className="input-error-text">{error}</span>}
    </div>
  )
}

export function Checkbox({ label, ...props }) {
  return (
    <label className="checkbox-container">
      <input type="checkbox" className="checkbox" {...props} />
      <span className="checkbox-label">{label}</span>
    </label>
  )
}

export function Radio({ label, ...props }) {
  return (
    <label className="radio-container">
      <input type="radio" className="radio" {...props} />
      <span className="radio-label">{label}</span>
    </label>
  )
}
