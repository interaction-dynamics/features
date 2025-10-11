import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { FeaturesProvider } from './components/features-provider'
import { ThemeProvider } from './components/theme-provider'
import { Dashboard } from './Dashboard'

const root = document.getElementById('root')

if (!root) {
  throw new Error('Failed to find the root element')
}

createRoot(root).render(
  <StrictMode>
    <ThemeProvider>
      <FeaturesProvider>
        <Dashboard />
      </FeaturesProvider>
    </ThemeProvider>
  </StrictMode>,
)
