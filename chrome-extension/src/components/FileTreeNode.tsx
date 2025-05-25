import React from 'react'
import { ChevronRight, ChevronDown, Folder, FolderOpen, Minus } from 'lucide-react'
import { useExtensionStore, FileTreeNode } from '../lib/store'
import { cn, getFileIcon, getDirectorySelectionState } from '../lib/utils'

interface FileTreeNodeProps {
  node: FileTreeNode
  level: number
  allFiles: string[]
  onToggleExpansion: (nodeId: string) => void
  onToggleSelection: (filePath: string) => void
  onToggleDirectorySelection: (directoryPath: string, allFiles: string[]) => void
}

export default function FileTreeNodeComponent({
  node,
  level,
  allFiles,
  onToggleExpansion,
  onToggleSelection,
  onToggleDirectorySelection,
}: FileTreeNodeProps) {
  const { selectedFiles } = useExtensionStore()
  const isSelected = selectedFiles.has(node.path)

  // For directories, check selection state
  const directorySelectionState = node.isDirectory
    ? getDirectorySelectionState(node.path, allFiles, selectedFiles)
    : 'none'

  const handleToggleExpansion = (e: React.MouseEvent) => {
    e.stopPropagation()
    if (node.isDirectory) {
      onToggleExpansion(node.id)
    }
  }

  const handleToggleSelection = () => {
    if (node.isDirectory) {
      onToggleDirectorySelection(node.path, allFiles)
    } else {
      onToggleSelection(node.path)
    }
  }

  return (
    <div>
      <div
        className={cn(
          'group hover:bg-cyber-700/30 flex cursor-pointer items-center rounded px-2 py-1.5 transition-colors',
          isSelected && !node.isDirectory && 'bg-neon-500/10 border-neon-500 border-l-2'
        )}
        style={{ paddingLeft: `${level * 16 + 8}px` }}
        onClick={handleToggleSelection}
      >
        {/* Expansion toggle for directories */}
        {node.isDirectory && (
          <button
            onClick={handleToggleExpansion}
            className="text-cyber-400 hover:text-neon-400 mr-1 flex h-4 w-4 items-center justify-center transition-colors"
          >
            {node.expanded ? (
              <ChevronDown className="h-3 w-3" />
            ) : (
              <ChevronRight className="h-3 w-3" />
            )}
          </button>
        )}

        {/* File/Folder icon */}
        <div className="mr-2 flex h-4 w-4 items-center justify-center">
          {node.isDirectory ? (
            node.expanded ? (
              <FolderOpen className="text-neon-400 h-3 w-3" />
            ) : (
              <Folder className="text-cyber-400 h-3 w-3" />
            )
          ) : (
            <span className="text-xs">{getFileIcon(node.name)}</span>
          )}
        </div>

        {/* Checkbox for files and directories */}
        <div className="mr-2 flex h-3 w-3 items-center justify-center">
          {node.isDirectory ? (
            // Directory checkbox with three states
            <div
              className={cn(
                'flex h-3 w-3 cursor-pointer items-center justify-center rounded border transition-colors',
                directorySelectionState === 'all'
                  ? 'bg-neon-500 border-neon-500'
                  : directorySelectionState === 'partial'
                    ? 'bg-neon-500/50 border-neon-500'
                    : 'border-cyber-500 bg-cyber-800'
              )}
              onClick={(e) => {
                e.stopPropagation()
                handleToggleSelection()
              }}
            >
              {directorySelectionState === 'all' && (
                <span className="text-xs font-bold text-white">✓</span>
              )}
              {directorySelectionState === 'partial' && <Minus className="h-2 w-2 text-white" />}
            </div>
          ) : (
            // File checkbox
            <input
              type="checkbox"
              checked={isSelected}
              onChange={handleToggleSelection}
              className="border-cyber-500 bg-cyber-800 text-neon-500 focus:ring-neon-500 h-3 w-3 rounded focus:ring-1"
              onClick={(e) => e.stopPropagation()}
            />
          )}
        </div>

        {/* File/Folder name */}
        <span
          className={cn(
            'flex-1 truncate text-sm',
            node.isDirectory
              ? 'text-cyber-300 font-medium'
              : isSelected
                ? 'text-neon-300'
                : 'text-cyber-200',
            'transition-colors group-hover:text-white'
          )}
        >
          {node.name}
        </span>

        {/* File extension badge */}
        {!node.isDirectory && node.name.includes('.') && (
          <span className="text-cyber-500 ml-2 font-mono text-xs">
            {node.name.split('.').pop()?.toUpperCase()}
          </span>
        )}
      </div>

      {/* Render children if directory is expanded */}
      {node.isDirectory && node.expanded && node.children && (
        <div>
          {node.children.map((child) => (
            <FileTreeNodeComponent
              key={child.id}
              node={child}
              level={level + 1}
              allFiles={allFiles}
              onToggleExpansion={onToggleExpansion}
              onToggleSelection={onToggleSelection}
              onToggleDirectorySelection={onToggleDirectorySelection}
            />
          ))}
        </div>
      )}
    </div>
  )
}
