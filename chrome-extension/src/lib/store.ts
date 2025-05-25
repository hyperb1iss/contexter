import { create } from 'zustand'
import { persist } from 'zustand/middleware'

export interface Project {
  name: string
  path: string
  files: string[]
}

export interface Settings {
  apiKey: string
  serverUrl: string
  theme: 'system' | 'light' | 'dark'
}

export interface FileTreeNode {
  id: string
  name: string
  path: string
  isDirectory: boolean
  children?: FileTreeNode[]
  selected: boolean
  expanded: boolean
}

interface ExtensionState {
  // Projects
  projects: Project[]
  currentProject: Project | null
  projectMetadata: Project | null

  // File selection
  selectedFiles: Set<string>
  fileTree: FileTreeNode[]

  // Content
  projectContent: string | null

  // UI state
  isLoading: boolean
  loadingMessage: string
  error: string | null

  // Settings
  settings: Settings

  // Actions
  setProjects: (projects: Project[]) => void
  setCurrentProject: (project: Project | null) => void
  setProjectMetadata: (metadata: Project | null) => void
  setSelectedFiles: (files: Set<string>) => void
  toggleFileSelection: (filePath: string) => void
  toggleDirectorySelection: (directoryPath: string, allFiles: string[]) => void
  selectAllFiles: () => void
  deselectAllFiles: () => void
  setFileTree: (tree: FileTreeNode[]) => void
  toggleNodeExpansion: (nodeId: string) => void
  setProjectContent: (content: string | null) => void
  setLoading: (loading: boolean, message?: string) => void
  setError: (error: string | null) => void
  updateSettings: (settings: Partial<Settings>) => void
}

export const useExtensionStore = create<ExtensionState>()(
  persist(
    (set, get) => ({
      // Initial state
      projects: [],
      currentProject: null,
      projectMetadata: null,
      selectedFiles: new Set(),
      fileTree: [],
      projectContent: null,
      isLoading: false,
      loadingMessage: '',
      error: null,
      settings: {
        apiKey: '',
        serverUrl: 'http://localhost:3030',
        theme: 'system',
      },

      // Actions
      setProjects: (projects) => set({ projects }),

      setCurrentProject: (project) => set({ currentProject: project }),

      setProjectMetadata: (metadata) =>
        set({
          projectMetadata: metadata,
          selectedFiles: new Set(), // Clear selections when switching projects
          fileTree: [], // Clear file tree
          projectContent: null, // Clear previous content
        }),

      setSelectedFiles: (files) => set({ selectedFiles: files }),

      toggleFileSelection: (filePath) => {
        const { selectedFiles } = get()
        const newSelection = new Set(selectedFiles)
        if (newSelection.has(filePath)) {
          newSelection.delete(filePath)
        } else {
          newSelection.add(filePath)
        }
        set({ selectedFiles: newSelection })
      },

      toggleDirectorySelection: (directoryPath, allFiles) => {
        const { selectedFiles } = get()
        const newSelection = new Set(selectedFiles)

        // Find all files in this directory (including subdirectories)
        const directoryFiles = allFiles.filter((file) => {
          // Exact match for the directory itself if it's a file
          if (file === directoryPath) return true
          // Files that are in this directory or subdirectories
          return file.startsWith(directoryPath + '/')
        })

        console.log('Directory selection for:', directoryPath)
        console.log('Found directory files:', directoryFiles)

        // Check if all directory files are already selected
        const allDirectoryFilesSelected =
          directoryFiles.length > 0 && directoryFiles.every((file) => selectedFiles.has(file))

        console.log('All selected?', allDirectoryFilesSelected)

        if (allDirectoryFilesSelected) {
          // Deselect all files in directory
          directoryFiles.forEach((file) => newSelection.delete(file))
        } else {
          // Select all files in directory
          directoryFiles.forEach((file) => newSelection.add(file))
        }

        console.log('New selection size:', newSelection.size)
        set({ selectedFiles: newSelection })
      },

      selectAllFiles: () => {
        const { projectMetadata } = get()
        if (projectMetadata) {
          set({ selectedFiles: new Set(projectMetadata.files) })
        }
      },

      deselectAllFiles: () => set({ selectedFiles: new Set() }),

      setFileTree: (tree) => set({ fileTree: tree }),

      toggleNodeExpansion: (nodeId) => {
        const { fileTree } = get()
        const updateNode = (nodes: FileTreeNode[]): FileTreeNode[] =>
          nodes.map((node) =>
            node.id === nodeId
              ? { ...node, expanded: !node.expanded }
              : node.children
                ? { ...node, children: updateNode(node.children) }
                : node
          )
        set({ fileTree: updateNode(fileTree) })
      },

      setProjectContent: (content) => set({ projectContent: content }),

      setLoading: (loading, message = '') => set({ isLoading: loading, loadingMessage: message }),

      setError: (error) => set({ error }),

      updateSettings: (newSettings) =>
        set((state) => ({
          settings: { ...state.settings, ...newSettings },
        })),
    }),
    {
      name: 'contexter-storage',
      partialize: (state) => ({ settings: state.settings }),
    }
  )
)
