import { useState } from 'react';
import { CATEGORIES } from '../data/categories';
import api from '../services/api';
import './FreelancerProfileSetup.css';

function FreelancerProfileSetup() {
  const [step, setStep] = useState(1);
  const [profile, setProfile] = useState({
    category: '',
    specializations: [],
    niches: [],
    hourlyRate: '',
    projectPricing: '',
    location: '',
    bio: '',
    skills: [],
    portfolio: [],
    availability: 'available',
  });

  const [selectedCategory, setSelectedCategory] = useState(null);
  const [selectedSpecialization, setSelectedSpecialization] = useState(null);

  const handleCategorySelect = (category) => {
    setSelectedCategory(category);
    setProfile({ ...profile, category: category.id, specializations: [], niches: [] });
  };

  const handleSpecializationSelect = (spec) => {
    setSelectedSpecialization(spec);
    const specializations = profile.specializations.includes(spec.id)
      ? profile.specializations.filter(s => s !== spec.id)
      : [...profile.specializations, spec.id];
    setProfile({ ...profile, specializations });
  };

  const handleNicheToggle = (niche) => {
    const niches = profile.niches.includes(niche)
      ? profile.niches.filter(n => n !== niche)
      : [...profile.niches, niche];
    setProfile({ ...profile, niches });
  };

  const handleSubmit = async () => {
    try {
      await api.freelancer.updateProfile(profile);
      alert('Profile updated successfully!');
    } catch (error) {
      console.error('Failed to update profile:', error);
      alert('Failed to update profile');
    }
  };

  const renderStep1 = () => (
    <div className="setup-step">
      <h2>Choose Your Main Category</h2>
      <p className="step-description">Select the category that best represents your expertise</p>

      <div className="categories-grid">
        {CATEGORIES.map((category) => (
          <div
            key={category.id}
            className={`category-card ${selectedCategory?.id === category.id ? 'selected' : ''}`}
            onClick={() => handleCategorySelect(category)}
          >
            <div className="category-icon">{category.icon}</div>
            <h3>{category.name}</h3>
            <p>{category.specializations.length} specializations</p>
          </div>
        ))}
      </div>

      <button
        className="btn btn-primary btn-next"
        onClick={() => setStep(2)}
        disabled={!selectedCategory}
      >
        Next: Choose Specializations →
      </button>
    </div>
  );

  const renderStep2 = () => (
    <div className="setup-step">
      <h2>Select Your Specializations</h2>
      <p className="step-description">
        Choose one or more specializations within {selectedCategory?.name}
      </p>

      <div className="specializations-grid">
        {selectedCategory?.specializations.map((spec) => (
          <div
            key={spec.id}
            className={`specialization-card ${profile.specializations.includes(spec.id) ? 'selected' : ''}`}
            onClick={() => handleSpecializationSelect(spec)}
          >
            <h4>{spec.name}</h4>
            <p>{spec.niches.length} niches available</p>
            {profile.specializations.includes(spec.id) && <div className="check-mark">✓</div>}
          </div>
        ))}
      </div>

      <div className="step-actions">
        <button className="btn btn-secondary" onClick={() => setStep(1)}>
          ← Back
        </button>
        <button
          className="btn btn-primary"
          onClick={() => setStep(3)}
          disabled={profile.specializations.length === 0}
        >
          Next: Define Your Niches →
        </button>
      </div>
    </div>
  );

  const renderStep3 = () => {
    const selectedSpecs = selectedCategory?.specializations.filter(s =>
      profile.specializations.includes(s.id)
    ) || [];

    return (
      <div className="setup-step">
        <h2>Define Your Specific Niches</h2>
        <p className="step-description">
          This helps clients find exactly the right expert. For example, if you're a transcriptionist,
          specify whether you focus on legal, medical, or law enforcement transcription.
        </p>

        {selectedSpecs.map((spec) => (
          <div key={spec.id} className="niche-section">
            <h3>{spec.name}</h3>
            <div className="niches-grid">
              {spec.niches.map((niche) => (
                <button
                  key={niche}
                  className={`niche-tag ${profile.niches.includes(niche) ? 'selected' : ''}`}
                  onClick={() => handleNicheToggle(niche)}
                >
                  {niche}
                  {profile.niches.includes(niche) && <span className="tag-check">✓</span>}
                </button>
              ))}
            </div>
          </div>
        ))}

        <div className="step-actions">
          <button className="btn btn-secondary" onClick={() => setStep(2)}>
            ← Back
          </button>
          <button
            className="btn btn-primary"
            onClick={() => setStep(4)}
            disabled={profile.niches.length === 0}
          >
            Next: Set Your Rates →
          </button>
        </div>
      </div>
    );
  };

  const renderStep4 = () => (
    <div className="setup-step">
      <h2>Set Your Rates & Availability</h2>

      <div className="form-grid">
        <div className="form-group">
          <label>Hourly Rate (USD)</label>
          <input
            type="number"
            value={profile.hourlyRate}
            onChange={(e) => setProfile({ ...profile, hourlyRate: e.target.value })}
            placeholder="e.g., 50"
            className="form-input"
          />
        </div>

        <div className="form-group">
          <label>Project Pricing (Optional)</label>
          <input
            type="number"
            value={profile.projectPricing}
            onChange={(e) => setProfile({ ...profile, projectPricing: e.target.value })}
            placeholder="e.g., 5000"
            className="form-input"
          />
        </div>

        <div className="form-group">
          <label>Location</label>
          <input
            type="text"
            value={profile.location}
            onChange={(e) => setProfile({ ...profile, location: e.target.value })}
            placeholder="e.g., Lagos, Nigeria"
            className="form-input"
          />
        </div>

        <div className="form-group">
          <label>Availability Status</label>
          <select
            value={profile.availability}
            onChange={(e) => setProfile({ ...profile, availability: e.target.value })}
            className="form-select"
          >
            <option value="available">Available Now</option>
            <option value="busy">Busy</option>
            <option value="unavailable">Unavailable</option>
          </select>
        </div>
      </div>

      <div className="form-group">
        <label>Professional Bio</label>
        <textarea
          value={profile.bio}
          onChange={(e) => setProfile({ ...profile, bio: e.target.value })}
          placeholder="Tell clients about your experience, expertise, and what makes you stand out..."
          className="form-textarea"
          rows="6"
        />
      </div>

      <div className="step-actions">
        <button className="btn btn-secondary" onClick={() => setStep(3)}>
          ← Back
        </button>
        <button className="btn btn-primary" onClick={handleSubmit}>
          Complete Profile ✓
        </button>
      </div>
    </div>
  );

  const renderProgressBar = () => (
    <div className="progress-bar">
      <div className="progress-steps">
        {[1, 2, 3, 4].map((s) => (
          <div key={s} className={`progress-step ${step >= s ? 'active' : ''}`}>
            <div className="step-number">{s}</div>
            <div className="step-label">
              {s === 1 && 'Category'}
              {s === 2 && 'Specialization'}
              {s === 3 && 'Niches'}
              {s === 4 && 'Rates'}
            </div>
          </div>
        ))}
      </div>
    </div>
  );

  return (
    <div className="freelancer-profile-setup">
      <div className="setup-header">
        <h1>🎯 Complete Your Freelancer Profile</h1>
        <p>Help clients find you by defining your exact expertise and specializations</p>
      </div>

      {renderProgressBar()}

      <div className="setup-content">
        {step === 1 && renderStep1()}
        {step === 2 && renderStep2()}
        {step === 3 && renderStep3()}
        {step === 4 && renderStep4()}
      </div>
    </div>
  );
}

export default FreelancerProfileSetup;
