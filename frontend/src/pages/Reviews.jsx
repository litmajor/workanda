import { useState } from 'react'
import './Reviews.css'

function Reviews() {
  const [activeTab, setActiveTab] = useState('received')

  const receivedReviews = [
    {
      id: 1,
      client: 'Sarah Chen',
      project: 'E-commerce Platform Development',
      rating: 5,
      review: 'Outstanding work! Alex delivered the project ahead of schedule and the quality exceeded expectations. Communication was excellent throughout.',
      date: '2 weeks ago',
      helpful: 12
    },
    {
      id: 2,
      client: 'Mike Rodriguez',
      project: 'API Development',
      rating: 5,
      review: 'Great developer with strong technical skills. The API was well-documented and performant. Will definitely work with again!',
      date: '1 month ago',
      helpful: 8
    },
    {
      id: 3,
      client: 'Emily Watson',
      project: 'React Dashboard',
      rating: 4,
      review: 'Good work overall. The dashboard looks great and functions well. Minor delays in communication but the end result was solid.',
      date: '2 months ago',
      helpful: 5
    }
  ]

  const givenReviews = [
    {
      id: 1,
      freelancer: 'John Smith',
      project: 'Design Consultation',
      rating: 5,
      review: 'Excellent client! Clear requirements and timely payments. A pleasure to work with.',
      date: '3 weeks ago'
    }
  ]

  const stats = {
    averageRating: 4.8,
    totalReviews: receivedReviews.length,
    fiveStars: receivedReviews.filter(r => r.rating === 5).length,
    fourStars: receivedReviews.filter(r => r.rating === 4).length
  }

  const renderStars = (rating) => {
    return '⭐'.repeat(rating) + '☆'.repeat(5 - rating)
  }

  return (
    <div className="reviews-page">
      <div className="reviews-container">
        <div className="reviews-header">
          <div>
            <h1>Reviews & Ratings</h1>
            <p>Your reputation score based on client feedback</p>
          </div>
          <div className="rating-summary">
            <div className="big-rating">
              <span className="rating-number">{stats.averageRating}</span>
              <div className="rating-stars">{renderStars(Math.round(stats.averageRating))}</div>
            </div>
            <div className="rating-details">
              <p>{stats.totalReviews} reviews</p>
              <div className="rating-bars">
                <div className="rating-bar-row">
                  <span>5 ⭐</span>
                  <div className="rating-bar">
                    <div
                      className="rating-bar-fill"
                      style={{ width: `${(stats.fiveStars / stats.totalReviews) * 100}%` }}
                    />
                  </div>
                  <span>{stats.fiveStars}</span>
                </div>
                <div className="rating-bar-row">
                  <span>4 ⭐</span>
                  <div className="rating-bar">
                    <div
                      className="rating-bar-fill"
                      style={{ width: `${(stats.fourStars / stats.totalReviews) * 100}%` }}
                    />
                  </div>
                  <span>{stats.fourStars}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="reviews-tabs">
          <button
            className={`tab ${activeTab === 'received' ? 'active' : ''}`}
            onClick={() => setActiveTab('received')}
          >
            Received ({receivedReviews.length})
          </button>
          <button
            className={`tab ${activeTab === 'given' ? 'active' : ''}`}
            onClick={() => setActiveTab('given')}
          >
            Given ({givenReviews.length})
          </button>
        </div>

        {activeTab === 'received' && (
          <div className="reviews-list">
            {receivedReviews.map(review => (
              <div key={review.id} className="review-card">
                <div className="review-header">
                  <div className="reviewer-info">
                    <div className="reviewer-avatar">{review.client[0]}</div>
                    <div>
                      <h3>{review.client}</h3>
                      <p className="review-project">{review.project}</p>
                    </div>
                  </div>
                  <div className="review-meta">
                    <div className="review-rating">{renderStars(review.rating)}</div>
                    <span className="review-date">{review.date}</span>
                  </div>
                </div>
                <p className="review-text">{review.review}</p>
                <div className="review-footer">
                  <button className="helpful-btn">
                    👍 Helpful ({review.helpful})
                  </button>
                  <button className="btn btn-secondary btn-small">Report</button>
                </div>
              </div>
            ))}
          </div>
        )}

        {activeTab === 'given' && (
          <div className="reviews-list">
            {givenReviews.map(review => (
              <div key={review.id} className="review-card">
                <div className="review-header">
                  <div className="reviewer-info">
                    <div className="reviewer-avatar">{review.freelancer[0]}</div>
                    <div>
                      <h3>{review.freelancer}</h3>
                      <p className="review-project">{review.project}</p>
                    </div>
                  </div>
                  <div className="review-meta">
                    <div className="review-rating">{renderStars(review.rating)}</div>
                    <span className="review-date">{review.date}</span>
                  </div>
                </div>
                <p className="review-text">{review.review}</p>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}

export default Reviews
