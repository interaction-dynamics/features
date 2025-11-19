import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { HashRouter, Route, Routes } from 'react-router'

import './index.css'
import { FeaturesProvider } from './components/features-provider'
import { MainLayout } from './components/main-layout'
import { ThemeProvider } from './components/theme-provider'
import { Features } from './routes/features'
import { Insights } from './routes/insights'

const root = document.getElementById('root')

if (!root) {
  throw new Error('Failed to find the root element')
}

createRoot(root).render(
  <StrictMode>
    <ThemeProvider>
      <FeaturesProvider>
        <HashRouter>
          <Routes>
            <Route element={<MainLayout />}>
              <Route path="/" element={<Features />} />
              <Route path="/insights" element={<Insights />} />
            </Route>
          </Routes>
        </HashRouter>
      </FeaturesProvider>
    </ThemeProvider>
  </StrictMode>,
)
