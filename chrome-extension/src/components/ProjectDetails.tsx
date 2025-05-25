import { ArrowLeft, Folder, FileText } from 'lucide-react'
import { useExtensionStore } from '../lib/store'
import FileTree from './FileTree'
import FileSelectionActions from './FileSelectionActions'

export default function ProjectDetails() {
  const { currentProject, projectMetadata, selectedFiles, setCurrentProject } = useExtensionStore()

  if (!currentProject) {
    return null
  }

  const totalFiles = projectMetadata?.files?.length || 0
  const selectedCount = selectedFiles.size

  return (
    <div className="flex h-full flex-1 flex-col">
      {/* Header */}
      <div className="border-cyber-600/30 flex-shrink-0 border-b p-4">
        <button
          onClick={() => setCurrentProject(null)}
          className="text-cyber-300 hover:text-neon-400 mb-3 flex items-center transition-colors"
        >
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Projects
        </button>

        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-lg font-semibold text-white">{currentProject.name}</h2>
            <p className="text-cyber-400 mt-1 font-mono text-sm">{currentProject.path}</p>
          </div>

          <div className="flex items-center space-x-4 text-sm">
            <div className="text-cyber-300 flex items-center space-x-2">
              <Folder className="h-4 w-4" />
              <span>{totalFiles} files</span>
            </div>
            {selectedCount > 0 && (
              <div className="text-neon-400 flex items-center space-x-2">
                <FileText className="h-4 w-4" />
                <span>{selectedCount} selected</span>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* File Tree */}
      <div className="flex-1 overflow-hidden">
        <FileTree />
      </div>

      {/* Selection Actions */}
      <div className="flex-shrink-0">
        <FileSelectionActions />
      </div>
    </div>
  )
}
