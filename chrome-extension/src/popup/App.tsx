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
      "background: linear-gradient(90deg, #000033 0%, #0033cc 50%, #6600cc 100%); color: #00ffff; font-weight: bold; padding: 5px 10px; border-radius: 5px; text-shadow: 0 0 5px #fff, 0 0 10px #fff, 0 0 15px #fff, 0 0 20px #00ffff, 0 0 35px #00ffff, 0 0 40px #00ffff, 0 0 50px #00ffff, 0 0 75px #00ffff;"
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
      const errorMessage = err instanceof ApiError 
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
      <div className="w-full h-full bg-gradient-to-br from-cyber-900 via-cyber-800 to-cyber-900 flex items-center justify-center p-6">
        <div className="text-center space-y-4">
          <div className="w-16 h-16 mx-auto bg-gradient-to-r from-neon-500 to-electric-600 rounded-full flex items-center justify-center">
            <Zap className="w-8 h-8 text-white" />
          </div>
          <h2 className="text-xl font-semibold gradient-text">Welcome to Contexter</h2>
          <p className="text-cyber-300 text-sm max-w-xs">
            Please configure your API key and server URL to get started.
          </p>
          <button
            onClick={() => chrome.runtime.openOptionsPage()}
            className="px-4 py-2 bg-neon-600 hover:bg-neon-500 text-white rounded-lg transition-colors glow-border"
          >
            Open Settings
          </button>
        </div>
      </div>
    )
  }

  return (
    <ErrorBoundary>
      <div className="w-full h-full bg-gradient-to-br from-cyber-900 via-cyber-800 to-cyber-900 text-white overflow-hidden">
        {/* Background Pattern */}
        <div className="absolute inset-0 opacity-5">
          <div className="absolute inset-0" style={{
            backgroundImage: `radial-gradient(circle at 25% 25%, rgba(59, 130, 246, 0.3) 0%, transparent 50%),
                             radial-gradient(circle at 75% 75%, rgba(124, 58, 237, 0.3) 0%, transparent 50%)`
          }} />
        </div>

        <div className="relative z-10 flex flex-col h-full">
          <Header onRefresh={loadProjects} />
          
          <main className="flex-1 overflow-hidden">
            {error && (
              <div className="mx-4 mb-4 p-3 bg-red-500/20 border border-red-500/30 rounded-lg text-red-300 text-sm">
                {error}
              </div>
            )}

            {currentProject ? (
              <ProjectDetails />
            ) : (
              <ProjectList projects={projects} />
            )}
          </main>
        </div>

        {isLoading && <LoadingOverlay />}
      </div>
    </ErrorBoundary>
  )
}

export default App 