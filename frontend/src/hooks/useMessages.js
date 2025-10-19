
import { useState, useEffect } from 'react'
import { messagesAPI } from '../services/api'
import { useApp } from '../context/AppContext'

export const useConversations = () => {
  const [conversations, setConversations] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchConversations = async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await messagesAPI.getConversations()
      setConversations(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch conversations')
      throw err
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchConversations()
  }, [])

  return { conversations, loading, error, refetch: fetchConversations }
}

export const useMessages = (userId) => {
  const [messages, setMessages] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(null)
  const { addAlert } = useApp()

  const fetchMessages = async () => {
    if (!userId) return
    setLoading(true)
    setError(null)
    try {
      const data = await messagesAPI.getMessages(userId)
      setMessages(data)
      return data
    } catch (err) {
      setError(err.message)
      addAlert('error', err.message || 'Failed to fetch messages')
      throw err
    } finally {
      setLoading(false)
    }
  }

  const sendMessage = async (content, attachments = []) => {
    try {
      const data = await messagesAPI.sendMessage(userId, content, attachments)
      setMessages([...messages, data])
      return data
    } catch (err) {
      addAlert('error', err.message || 'Failed to send message')
      throw err
    }
  }

  const markAsRead = async (messageId) => {
    try {
      await messagesAPI.markAsRead(messageId)
      setMessages(messages.map(m => 
        m.id === messageId ? { ...m, read: true } : m
      ))
    } catch (err) {
      addAlert('error', err.message || 'Failed to mark as read')
      throw err
    }
  }

  useEffect(() => {
    fetchMessages()
  }, [userId])

  return { messages, loading, error, sendMessage, markAsRead, refetch: fetchMessages }
}
