import { createRoot } from 'react-dom/client'
import App from './App'
import '../styles/globals.css'

const container = document.getElementById('popup-root')
if (container) {
  const root = createRoot(container)
  root.render(<App />)
} else {
  console.error('Popup root container not found')
} 