import { useEffect } from 'react'
import { useExtensionStore } from '../lib/store'
import { api, ApiError } from '../lib/api'
import Header from '../components/Header'
import ProjectList from '../components/ProjectList'
import ProjectDetails from '../components/ProjectDetails'
import LoadingOverlay from '../components/LoadingOverlay'
import ErrorBoundary from '../components/ErrorBoundary'
import { Zap } from 'lucide-react'

function App() {
  const {
    projects,
    currentProject,
    isLoading,
    error,
    settings,
    setProjects,
    setLoading,
    setError,
  } = useExtensionStore()

  const showBanner = () => {
    console.log(
      `%c 🚀✨🌟 Contexter v1.0.0 launched! 🌠🛸🌈 `,
      'background: linear-gradient(90deg, #000033 0%, #0033cc 50%, #6600cc 100%); color: #00ffff; font-weight: bold; padding: 5px 10px; border-radius: 5px; text-shadow: 0 0 5px #fff, 0 0 10px #fff, 0 0 15px #fff, 0 0 20px #00ffff, 0 0 35px #00ffff, 0 0 40px #00ffff, 0 0 50px #00ffff, 0 0 75px #00ffff;'
    )
  }

  const loadProjects = async () => {
    if (!settings.apiKey || !settings.serverUrl) {
      setError('Please configure your API key and server URL in settings')
      return
    }

    setLoading(true, 'Loading projects...')
    setError(null)

    try {
      const projectList = await api.fetchProjects(settings)
      setProjects(projectList)

      if (projectList.length === 0) {
        setError('No projects found. Make sure your server is running and configured correctly.')
      }
    } catch (err) {
      const errorMessage =
        err instanceof ApiError
          ? `API Error: ${err.message}`
          : 'Failed to load projects. Please check your connection.'
      setError(errorMessage)
      console.error('Failed to load projects:', err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    showBanner()
    loadProjects()
  }, [settings.apiKey, settings.serverUrl])

  if (!settings.apiKey || !settings.serverUrl) {
    return (
      <div className="from-cyber-900 via-cyber-800 to-cyber-900 flex h-full w-full items-center justify-center bg-gradient-to-br p-6">
        <div className="space-y-4 text-center">
          <div className="from-neon-500 to-electric-600 mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-gradient-to-r">
            <Zap className="h-8 w-8 text-white" />
          </div>
          <h2 className="gradient-text text-xl font-semibold">Welcome to Contexter</h2>
          <p className="text-cyber-300 max-w-xs text-sm">
            Please configure your API key and server URL to get started.
          </p>
          <button
            onClick={() => chrome.runtime.openOptionsPage()}
            className="glow-border bg-neon-600 hover:bg-neon-500 rounded-lg px-4 py-2 text-white transition-colors"
          >
            Open Settings
          </button>
        </div>
      </div>
    )
  }

  return (
    <ErrorBoundary>
      <div className="from-cyber-900 via-cyber-800 to-cyber-900 h-full w-full overflow-hidden bg-gradient-to-br text-white">
        {/* Background Pattern */}
        <div className="absolute inset-0 opacity-5">
          <div
            className="absolute inset-0"
            style={{
              backgroundImage: `radial-gradient(circle at 25% 25%, rgba(59, 130, 246, 0.3) 0%, transparent 50%),
                             radial-gradient(circle at 75% 75%, rgba(124, 58, 237, 0.3) 0%, transparent 50%)`,
            }}
          />
        </div>

        <div className="relative z-10 flex h-full flex-col">
          <Header onRefresh={loadProjects} />

          <main className="flex-1 overflow-hidden">
            {error && (
              <div className="mx-4 mb-4 rounded-lg border border-red-500/30 bg-red-500/20 p-3 text-sm text-red-300">
                {error}
              </div>
            )}

            {currentProject ? <ProjectDetails /> : <ProjectList projects={projects} />}
          </main>
        </div>

        {isLoading && <LoadingOverlay />}
      </div>
    </ErrorBoundary>
  )
}

export default App
