
import { useState, useRef } from 'react'
import './FileUpload.css'

export function FileUpload({ onFilesSelected, accept, multiple = true, maxSize = 10485760 }) {
  const [isDragging, setIsDragging] = useState(false)
  const [files, setFiles] = useState([])
  const [error, setError] = useState('')
  const fileInputRef = useRef(null)

  const handleDragOver = (e) => {
    e.preventDefault()
    setIsDragging(true)
  }

  const handleDragLeave = (e) => {
    e.preventDefault()
    setIsDragging(false)
  }

  const validateFiles = (fileList) => {
    const validFiles = []
    const errors = []

    Array.from(fileList).forEach(file => {
      if (file.size > maxSize) {
        errors.push(`${file.name} exceeds ${maxSize / 1024 / 1024}MB limit`)
      } else {
        validFiles.push(file)
      }
    })

    if (errors.length > 0) {
      setError(errors.join(', '))
    } else {
      setError('')
    }

    return validFiles
  }

  const handleDrop = (e) => {
    e.preventDefault()
    setIsDragging(false)

    const droppedFiles = e.dataTransfer.files
    const validFiles = validateFiles(droppedFiles)
    
    if (validFiles.length > 0) {
      setFiles(prev => [...prev, ...validFiles])
      onFilesSelected?.(validFiles)
    }
  }

  const handleFileSelect = (e) => {
    const selectedFiles = e.target.files
    const validFiles = validateFiles(selectedFiles)
    
    if (validFiles.length > 0) {
      setFiles(prev => [...prev, ...validFiles])
      onFilesSelected?.(validFiles)
    }
  }

  const removeFile = (index) => {
    setFiles(prev => prev.filter((_, i) => i !== index))
  }

  const formatFileSize = (bytes) => {
    if (bytes === 0) return '0 Bytes'
    const k = 1024
    const sizes = ['Bytes', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
  }

  return (
    <div className="file-upload-container">
      <div
        className={`file-upload-dropzone ${isDragging ? 'dragging' : ''}`}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onClick={() => fileInputRef.current?.click()}
      >
        <div className="file-upload-icon">📁</div>
        <p className="file-upload-text">
          Drag & drop files here or <span className="file-upload-link">browse</span>
        </p>
        <p className="file-upload-hint">
          Max file size: {maxSize / 1024 / 1024}MB
        </p>
        <input
          ref={fileInputRef}
          type="file"
          accept={accept}
          multiple={multiple}
          onChange={handleFileSelect}
          className="file-upload-input"
        />
      </div>

      {error && <div className="file-upload-error">{error}</div>}

      {files.length > 0 && (
        <div className="file-upload-list">
          <h4>Selected Files ({files.length})</h4>
          {files.map((file, index) => (
            <div key={index} className="file-upload-item">
              <div className="file-upload-item-info">
                <span className="file-upload-item-icon">📄</span>
                <div>
                  <p className="file-upload-item-name">{file.name}</p>
                  <p className="file-upload-item-size">{formatFileSize(file.size)}</p>
                </div>
              </div>
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation()
                  removeFile(index)
                }}
                className="file-upload-remove"
              >
                ✕
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
