
import { useEffect, useRef } from 'react'
import { wsService } from '../services/api'
import { useApp } from '../context/AppContext'

export const useWebSocket = () => {
  const { addNotification } = useApp()
  const wsRef = useRef(null)

  useEffect(() => {
    // Connect to WebSocket
    wsService.connect()
    wsRef.current = wsService

    // Set up event listeners
    wsService.on('NewMessage', (data) => {
      addNotification({
        id: Date.now(),
        type: 'message',
        title: 'New Message',
        message: data.content,
        read: false,
        timestamp: new Date()
      })
    })

    wsService.on('NewNotification', (data) => {
      addNotification({
        id: Date.now(),
        type: 'notification',
        title: data.title,
        message: data.message,
        read: false,
        timestamp: new Date()
      })
    })

    wsService.on('ProjectUpdate', (data) => {
      addNotification({
        id: Date.now(),
        type: 'project',
        title: 'Project Update',
        message: data.message,
        read: false,
        timestamp: new Date()
      })
    })

    wsService.on('MilestoneUpdate', (data) => {
      addNotification({
        id: Date.now(),
        type: 'milestone',
        title: 'Milestone Update',
        message: data.message,
        read: false,
        timestamp: new Date()
      })
    })

    wsService.on('PaymentUpdate', (data) => {
      addNotification({
        id: Date.now(),
        type: 'payment',
        title: 'Payment Update',
        message: data.message,
        read: false,
        timestamp: new Date()
      })
    })

    // Cleanup on unmount
    return () => {
      wsService.disconnect()
    }
  }, [])

  const sendMessage = (type, data) => {
    if (wsRef.current) {
      wsRef.current.send(type, data)
    }
  }

  return { sendMessage }
}
