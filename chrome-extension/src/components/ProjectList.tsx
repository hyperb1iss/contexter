import { FolderOpen, ChevronRight } from 'lucide-react'
import { useExtensionStore, Project } from '../lib/store'
import { api } from '../lib/api'
import { cn, truncateText } from '../lib/utils'

interface ProjectListProps {
  projects: Project[]
}

export default function ProjectList({ projects }: ProjectListProps) {
  const { 
    settings, 
    setCurrentProject, 
    setProjectMetadata, 
    setLoading, 
    setError 
  } = useExtensionStore()

  const handleProjectSelect = async (project: Project) => {
    setLoading(true, `Loading ${project.name}...`)
    setError(null)

    try {
      const metadata = await api.fetchProjectMetadata(project.name, settings)
      setProjectMetadata(metadata)
      setCurrentProject(project)
    } catch (err) {
      setError(`Failed to load project: ${err instanceof Error ? err.message : 'Unknown error'}`)
      console.error('Failed to load project metadata:', err)
    } finally {
      setLoading(false)
    }
  }

  if (projects.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center p-8">
        <div className="text-center space-y-4">
          <div className="w-16 h-16 mx-auto bg-cyber-700/50 rounded-full flex items-center justify-center">
            <FolderOpen className="w-8 h-8 text-cyber-400" />
          </div>
          <p className="text-cyber-400 text-sm">No projects available</p>
          <p className="text-cyber-500 text-xs max-w-xs">
            Make sure your Contexter server is running and has projects configured.
          </p>
        </div>
      </div>
    )
  }

  return (
    <div className="flex-1 overflow-y-auto p-4 space-y-3">
      <h2 className="text-sm font-semibold text-cyber-300 mb-4 flex items-center">
        <FolderOpen className="w-4 h-4 mr-2" />
        Projects ({projects.length})
      </h2>
      
      {projects.map((project, index) => (
        <button
          key={project.name}
          onClick={() => handleProjectSelect(project)}
          className={cn(
            "w-full p-4 rounded-lg text-left transition-all duration-300 group",
            "glass-dark hover:bg-neon-500/10",
            "border border-cyber-600/30 hover:border-neon-500/50",
            "transform hover:scale-[1.02] hover:shadow-glow",
            "animate-float"
          )}
          style={{
            animationDelay: `${index * 0.1}s`
          }}
        >
          <div className="flex items-center justify-between">
            <div className="flex-1 min-w-0">
              <h3 className="font-semibold text-white group-hover:text-neon-300 transition-colors truncate">
                {project.name}
              </h3>
              <p className="text-xs text-cyber-400 group-hover:text-cyber-300 transition-colors mt-1 font-mono">
                {truncateText(project.path, 40)}
              </p>
              {project.files && (
                <p className="text-xs text-cyber-500 mt-2">
                  {project.files.length} files
                </p>
              )}
            </div>
            
            <ChevronRight className="w-5 h-5 text-cyber-500 group-hover:text-neon-400 group-hover:translate-x-1 transition-all duration-200 flex-shrink-0 ml-3" />
          </div>
          
          {/* Subtle gradient overlay on hover */}
          <div className="absolute inset-0 bg-gradient-to-r from-neon-500/0 via-neon-500/5 to-electric-600/0 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-lg" />
        </button>
      ))}
    </div>
  )
} 