import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'
import { FileTreeNode } from './store'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

export function getFileIcon(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase()

  switch (ext) {
    case 'ts':
    case 'tsx':
      return '🔷'
    case 'js':
    case 'jsx':
      return '🟨'
    case 'rs':
      return '🦀'
    case 'py':
      return '🐍'
    case 'go':
      return '🔵'
    case 'java':
      return '☕'
    case 'kt':
    case 'kts':
      return '🟣'
    case 'md':
      return '📝'
    case 'json':
      return '📋'
    case 'toml':
    case 'yaml':
    case 'yml':
      return '⚙️'
    case 'css':
    case 'scss':
    case 'sass':
    case 'less':
      return '🎨'
    case 'html':
    case 'htm':
      return '🌐'
    case 'xml':
      return '📄'
    case 'sql':
      return '🗃️'
    case 'sh':
    case 'bash':
    case 'zsh':
      return '⚡'
    case 'dockerfile':
      return '🐳'
    case 'gitignore':
    case 'gitattributes':
      return '📋'
    case 'lock':
      return '🔒'
    case 'log':
      return '📊'
    default:
      return '📄'
  }
}

export function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text
  return text.slice(0, maxLength) + '...'
}

export function buildFileTree(files: string[]): FileTreeNode[] {
  const root: FileTreeNode[] = []
  const nodeMap = new Map<string, FileTreeNode>()

  // Sort files to ensure consistent ordering
  const sortedFiles = [...files].sort()

  for (const filePath of sortedFiles) {
    const parts = filePath.split('/')
    let currentPath = ''
    let currentLevel = root

    for (let i = 0; i < parts.length; i++) {
      const part = parts[i]
      const isLastPart = i === parts.length - 1
      currentPath = currentPath ? `${currentPath}/${part}` : part

      // Check if node already exists at this level
      let existingNode = currentLevel.find((node) => node.name === part)

      if (!existingNode) {
        // Create new node
        const newNode: FileTreeNode = {
          id: currentPath,
          name: part,
          path: currentPath,
          isDirectory: !isLastPart,
          children: !isLastPart ? [] : undefined,
          selected: false,
          expanded: false,
        }

        existingNode = newNode
        currentLevel.push(newNode)
        nodeMap.set(currentPath, newNode)
      }

      // Move to next level if this is a directory
      if (!isLastPart && existingNode.children) {
        currentLevel = existingNode.children
      }
    }
  }

  return root
}

export function expandFileTreeDefaults(
  nodes: FileTreeNode[],
  maxDepth: number = 2,
  currentDepth: number = 0
): FileTreeNode[] {
  return nodes.map((node) => ({
    ...node,
    expanded: node.isDirectory && currentDepth < maxDepth,
    children: node.children
      ? expandFileTreeDefaults(node.children, maxDepth, currentDepth + 1)
      : undefined,
  }))
}

export function getDirectorySelectionState(
  directoryPath: string,
  allFiles: string[],
  selectedFiles: Set<string>
): 'none' | 'partial' | 'all' {
  const directoryFiles = allFiles.filter((file) => {
    // Exact match for the directory itself if it's a file
    if (file === directoryPath) return true
    // Files that are in this directory or subdirectories
    return file.startsWith(directoryPath + '/')
  })

  if (directoryFiles.length === 0) return 'none'

  const selectedCount = directoryFiles.filter((file) => selectedFiles.has(file)).length

  if (selectedCount === 0) return 'none'
  if (selectedCount === directoryFiles.length) return 'all'
  return 'partial'
}
