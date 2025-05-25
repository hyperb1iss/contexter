import { ArrowLeft } from 'lucide-react'
import { useExtensionStore } from '../lib/store'

export default function ProjectDetails() {
  const { currentProject, setCurrentProject } = useExtensionStore()

  return (
    <div className="flex flex-1 flex-col">
      <div className="border-b border-cyber-600/30 p-4">
        <button
          onClick={() => setCurrentProject(null)}
          className="flex items-center text-cyber-300 transition-colors hover:text-neon-400"
        >
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Projects
        </button>
        <h2 className="mt-2 text-lg font-semibold text-white">{currentProject?.name}</h2>
      </div>

      <div className="flex-1 p-4">
        <p className="text-cyber-400">Project details coming soon...</p>
      </div>
    </div>
  )
}
