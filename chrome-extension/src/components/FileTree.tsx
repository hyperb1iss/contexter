import React, { useState, useMemo } from 'react'
import { Search, X } from 'lucide-react'
import { useExtensionStore, FileTreeNode } from '../lib/store'
import { buildFileTree, expandFileTreeDefaults } from '../lib/utils'
import FileTreeNodeComponent from './FileTreeNode'
import { cn } from '../lib/utils'

export default function FileTree() {
  const {
    projectMetadata,
    fileTree,
    setFileTree,
    toggleNodeExpansion,
    toggleFileSelection,
    toggleDirectorySelection,
  } = useExtensionStore()

  const [searchQuery, setSearchQuery] = useState('')

  // Build initial file tree when project metadata changes
  React.useEffect(() => {
    if (projectMetadata?.files) {
      const tree = buildFileTree(projectMetadata.files)
      const expandedTree = expandFileTreeDefaults(tree, 1) // Only expand first level by default
      setFileTree(expandedTree)
    }
  }, [projectMetadata, setFileTree])

  // Filter tree based on search query
  const filteredTree = useMemo(() => {
    if (!searchQuery.trim()) return fileTree

    const filterNodes = (nodes: FileTreeNode[]): FileTreeNode[] => {
      return nodes.reduce((acc: FileTreeNode[], node) => {
        const matchesSearch =
          node.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          node.path.toLowerCase().includes(searchQuery.toLowerCase())

        if (node.isDirectory && node.children) {
          const filteredChildren = filterNodes(node.children)
          if (filteredChildren.length > 0 || matchesSearch) {
            acc.push({
              ...node,
              children: filteredChildren,
              expanded: filteredChildren.length > 0 || node.expanded, // Auto-expand if has matching children
            })
          }
        } else if (matchesSearch) {
          acc.push(node)
        }

        return acc
      }, [])
    }

    return filterNodes(fileTree)
  }, [fileTree, searchQuery])

  const clearSearch = () => {
    setSearchQuery('')
  }

  if (!projectMetadata?.files || projectMetadata.files.length === 0) {
    return (
      <div className="flex items-center justify-center p-8 text-center">
        <div className="space-y-2">
          <p className="text-cyber-400 text-sm">No files found in this project</p>
          <p className="text-cyber-500 text-xs">The project may be empty or still loading</p>
        </div>
      </div>
    )
  }

  return (
    <div className="flex h-full flex-col">
      {/* Search bar */}
      <div className="border-cyber-600/30 border-b p-4">
        <div className="relative">
          <Search className="text-cyber-400 absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 transform" />
          <input
            type="text"
            placeholder="Search files..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className={cn(
              'w-full rounded-lg py-2 pr-10 pl-10',
              'bg-cyber-800/50 border-cyber-600/30 border',
              'placeholder-cyber-400 text-white',
              'focus:border-neon-500/50 focus:ring-neon-500/50 focus:ring-1 focus:outline-none',
              'text-sm transition-colors'
            )}
          />
          {searchQuery && (
            <button
              onClick={clearSearch}
              className="text-cyber-400 hover:text-neon-400 absolute top-1/2 right-3 -translate-y-1/2 transform transition-colors"
            >
              <X className="h-4 w-4" />
            </button>
          )}
        </div>

        {searchQuery && (
          <p className="text-cyber-400 mt-2 text-xs">
            {filteredTree.length === 0 ? 'No matches found' : `Showing filtered results`}
          </p>
        )}
      </div>

      {/* File tree */}
      <div className="file-tree-scrollbar flex-1 overflow-auto p-2">
        {filteredTree.length === 0 && searchQuery ? (
          <div className="flex items-center justify-center p-8 text-center">
            <div className="space-y-2">
              <p className="text-cyber-400 text-sm">No files match your search</p>
              <button
                onClick={clearSearch}
                className="text-neon-400 hover:text-neon-300 text-xs transition-colors"
              >
                Clear search
              </button>
            </div>
          </div>
        ) : (
          <div className="space-y-1">
            {filteredTree.map((node) => (
              <FileTreeNodeComponent
                key={node.id}
                node={node}
                level={0}
                allFiles={projectMetadata?.files || []}
                onToggleExpansion={toggleNodeExpansion}
                onToggleSelection={toggleFileSelection}
                onToggleDirectorySelection={toggleDirectorySelection}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
