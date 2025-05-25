import { ArrowLeft } from 'lucide-react'
import { useExtensionStore } from '../lib/store'

export default function ProjectDetails() {
  const { currentProject, setCurrentProject } = useExtensionStore()

  return (
    <div className="flex-1 flex flex-col">
      <div className="p-4 border-b border-cyber-600/30">
        <button
          onClick={() => setCurrentProject(null)}
          className="flex items-center text-cyber-300 hover:text-neon-400 transition-colors"
        >
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back to Projects
        </button>
        <h2 className="text-lg font-semibold text-white mt-2">{currentProject?.name}</h2>
      </div>
      
      <div className="flex-1 p-4">
        <p className="text-cyber-400">Project details coming soon...</p>
      </div>
    </div>
  )
} 