
import './Skeleton.css'

function Skeleton({ variant = 'text', width, height, count = 1, className = '' }) {
  const skeletons = Array.from({ length: count }, (_, i) => i)

  const getSkeletonStyle = () => {
    const style = {}
    if (width) style.width = width
    if (height) style.height = height
    return style
  }

  return (
    <>
      {skeletons.map((_, index) => (
        <div
          key={index}
          className={`skeleton skeleton-${variant} ${className}`}
          style={getSkeletonStyle()}
        />
      ))}
    </>
  )
}

// Preset skeleton layouts
export function CardSkeleton() {
  return (
    <div className="skeleton-card">
      <Skeleton variant="rectangular" height="200px" />
      <div className="skeleton-card-content">
        <Skeleton variant="text" width="80%" />
        <Skeleton variant="text" width="60%" />
        <Skeleton variant="text" width="40%" />
      </div>
    </div>
  )
}

export function TableSkeleton({ rows = 5 }) {
  return (
    <div className="skeleton-table">
      {Array.from({ length: rows }).map((_, i) => (
        <div key={i} className="skeleton-table-row">
          <Skeleton variant="rectangular" height="40px" />
        </div>
      ))}
    </div>
  )
}

export function ProfileSkeleton() {
  return (
    <div className="skeleton-profile">
      <Skeleton variant="circular" width="100px" height="100px" />
      <div className="skeleton-profile-info">
        <Skeleton variant="text" width="200px" />
        <Skeleton variant="text" width="150px" />
        <Skeleton variant="text" width="300px" count={3} />
      </div>
    </div>
  )
}

export default Skeleton
