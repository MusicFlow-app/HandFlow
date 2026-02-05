const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080'

export const apiUrl = (path) => `${API_URL}${path}`

export default API_URL
