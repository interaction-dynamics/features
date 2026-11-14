import { lazy, StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { HashRouter, Navigate, Route, Routes } from 'react-router'
import './index.css'
import { FeaturesProvider } from './components/features-provider'
import { MainLayout } from './components/main-layout'
import { ThemeProvider } from './components/theme-provider'

const Features = lazy(async () => import('./routes/features'))

const Insights = lazy(async () => import('./routes/insights'))

const OutsideFeatures = lazy(async () => import('./routes/outside-features'))

const History = lazy(async () => import('./routes/history'))

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
              <Route path="/outside-features" element={<OutsideFeatures />} />
              {/* Former pages, merged into Outside features */}
              <Route
                path="/reconciliation"
                element={<Navigate to="/outside-features" replace />}
              />
              <Route
                path="/records"
                element={<Navigate to="/outside-features" replace />}
              />
              <Route path="/history" element={<History />} />
            </Route>
          </Routes>
        </HashRouter>
      </FeaturesProvider>
    </ThemeProvider>
  </StrictMode>,
)
