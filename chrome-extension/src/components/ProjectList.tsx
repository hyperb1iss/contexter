import { FolderOpen, ChevronRight } from 'lucide-react'
import { useExtensionStore, Project } from '../lib/store'
import { api } from '../lib/api'
import { cn, truncateText } from '../lib/utils'

interface ProjectListProps {
  projects: Project[]
}

export default function ProjectList({ projects }: ProjectListProps) {
  const { settings, setCurrentProject, setProjectMetadata, setLoading, setError } =
    useExtensionStore()

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
      <div className="flex flex-1 items-center justify-center p-8">
        <div className="space-y-4 text-center">
          <div className="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-cyber-700/50">
            <FolderOpen className="h-8 w-8 text-cyber-400" />
          </div>
          <p className="text-sm text-cyber-400">No projects available</p>
          <p className="max-w-xs text-xs text-cyber-500">
            Make sure your Contexter server is running and has projects configured.
          </p>
        </div>
      </div>
    )
  }

  return (
    <div className="flex-1 space-y-3 overflow-y-auto p-4">
      <h2 className="mb-4 flex items-center text-sm font-semibold text-cyber-300">
        <FolderOpen className="mr-2 h-4 w-4" />
        Projects ({projects.length})
      </h2>

      {projects.map((project, index) => (
        <button
          key={project.name}
          onClick={() => handleProjectSelect(project)}
          className={cn(
            'group w-full rounded-lg p-4 text-left transition-all duration-300',
            'glass-dark hover:bg-neon-500/10',
            'border border-cyber-600/30 hover:border-neon-500/50',
            'transform hover:scale-[1.02] hover:shadow-glow',
            'animate-float'
          )}
          style={{
            animationDelay: `${index * 0.1}s`,
          }}
        >
          <div className="flex items-center justify-between">
            <div className="min-w-0 flex-1">
              <h3 className="truncate font-semibold text-white transition-colors group-hover:text-neon-300">
                {project.name}
              </h3>
              <p className="mt-1 font-mono text-xs text-cyber-400 transition-colors group-hover:text-cyber-300">
                {truncateText(project.path, 40)}
              </p>
              {project.files && (
                <p className="mt-2 text-xs text-cyber-500">{project.files.length} files</p>
              )}
            </div>

            <ChevronRight className="ml-3 h-5 w-5 flex-shrink-0 text-cyber-500 transition-all duration-200 group-hover:translate-x-1 group-hover:text-neon-400" />
          </div>

          {/* Subtle gradient overlay on hover */}
          <div className="absolute inset-0 rounded-lg bg-gradient-to-r from-neon-500/0 via-neon-500/5 to-electric-600/0 opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        </button>
      ))}
    </div>
  )
}
