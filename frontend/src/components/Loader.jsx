
import './Loader.css'

function Loader({ size = 'medium', fullScreen = false }) {
  if (fullScreen) {
    return (
      <div className="loader-fullscreen">
        <div className={`spinner ${size}`}></div>
      </div>
    )
  }

  return <div className={`spinner ${size}`}></div>
}

export default Loader
