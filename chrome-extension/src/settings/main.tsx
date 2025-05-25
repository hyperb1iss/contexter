import { createRoot } from 'react-dom/client'
import Settings from './Settings'
import '../styles/globals.css'

const container = document.getElementById('settings-root')
if (container) {
  const root = createRoot(container)
  root.render(<Settings />)
} else {
  console.error('Settings root container not found')
}
