import { lazy, StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { HashRouter, Route, Routes } from 'react-router'
import './index.css'
import { ExperimentalRoute } from './components/experimental-route'
import { FeaturesProvider } from './components/features-provider'
import { MainLayout } from './components/main-layout'
import { ThemeProvider } from './components/theme-provider'

const Features = lazy(async () => import('./routes/features'))

const Insights = lazy(async () => import('./routes/insights'))

const Reconciliation = lazy(async () => import('./routes/reconciliation'))

const ProjectRecords = lazy(async () => import('./routes/project-records'))

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
              <Route path="/reconciliation" element={<Reconciliation />} />
              <Route
                path="/records"
                element={
                  <ExperimentalRoute>
                    <ProjectRecords />
                  </ExperimentalRoute>
                }
              />
              <Route
                path="/history"
                element={
                  <ExperimentalRoute>
                    <History />
                  </ExperimentalRoute>
                }
              />
            </Route>
          </Routes>
        </HashRouter>
      </FeaturesProvider>
    </ThemeProvider>
  </StrictMode>,
)
