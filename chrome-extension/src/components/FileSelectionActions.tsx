import { CheckSquare, Square, Copy, Download, FileText } from 'lucide-react'
import { useExtensionStore } from '../lib/store'
import { api } from '../lib/api'
import { cn } from '../lib/utils'

export default function FileSelectionActions() {
  const {
    selectedFiles,
    projectMetadata,
    settings,
    selectAllFiles,
    deselectAllFiles,
    setProjectContent,
    setLoading,
    setError,
  } = useExtensionStore()

  const totalFiles = projectMetadata?.files?.length || 0
  const selectedCount = selectedFiles.size
  const allSelected = selectedCount === totalFiles && totalFiles > 0

  const handleSelectAll = () => {
    if (allSelected) {
      deselectAllFiles()
    } else {
      selectAllFiles()
    }
  }

  const handleCopyToClipboard = async () => {
    if (!projectMetadata || selectedFiles.size === 0) {
      setError('No files selected')
      return
    }

    setLoading(true, 'Fetching file contents...')
    setError(null)

    try {
      const selectedFilesArray = Array.from(selectedFiles)
      const allFilesArray = projectMetadata.files

      console.log('Copy: Selected files:', selectedFilesArray)
      console.log('Copy: All files:', allFilesArray)

      const content = await api.fetchProjectContent(
        projectMetadata.name,
        selectedFilesArray,
        allFilesArray,
        settings
      )

      console.log('Copy: Content length:', content.length)
      console.log('Copy: Content preview:', content.substring(0, 200))

      if (!content || content.length === 0) {
        setError('No content received from server')
        return
      }

      await navigator.clipboard.writeText(content)
      setProjectContent(content)

      // Show success message temporarily
      setLoading(false, '')
      setTimeout(() => {
        setError(`✅ Copied ${selectedCount} files to clipboard!`)
        setTimeout(() => setError(null), 2000)
      }, 100)
    } catch (err) {
      setError(`Failed to copy files: ${err instanceof Error ? err.message : 'Unknown error'}`)
      console.error('Failed to copy files:', err)
    } finally {
      setLoading(false)
    }
  }

  const handleDownload = async () => {
    if (!projectMetadata || selectedFiles.size === 0) {
      setError('No files selected')
      return
    }

    setLoading(true, 'Preparing download...')
    setError(null)

    try {
      const selectedFilesArray = Array.from(selectedFiles)
      const allFilesArray = projectMetadata.files

      console.log('Download: Selected files:', selectedFilesArray)
      console.log('Download: All files:', allFilesArray)

      const content = await api.fetchProjectContent(
        projectMetadata.name,
        selectedFilesArray,
        allFilesArray,
        settings
      )

      console.log('Download: Content length:', content.length)
      console.log('Download: Content preview:', content.substring(0, 200))

      if (!content || content.length === 0) {
        setError('No content received from server')
        return
      }

      // Create and trigger download
      const blob = new Blob([content], { type: 'text/plain;charset=utf-8' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `${projectMetadata.name}-context.txt`
      a.style.display = 'none'
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)

      setProjectContent(content)

      // Show success message
      setTimeout(() => {
        setError(`✅ Downloaded ${selectedCount} files!`)
        setTimeout(() => setError(null), 2000)
      }, 100)
    } catch (err) {
      setError(`Failed to download files: ${err instanceof Error ? err.message : 'Unknown error'}`)
      console.error('Failed to download files:', err)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="border-cyber-600/30 bg-cyber-800/30 space-y-3 border-t p-4">
      {/* Selection summary */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <FileText className="text-cyber-400 h-4 w-4" />
          <span className="text-cyber-300 text-sm">
            {selectedCount} of {totalFiles} files selected
          </span>
        </div>

        {selectedCount > 0 && (
          <span className="text-neon-400 font-mono text-xs">
            {Math.round((selectedCount / totalFiles) * 100)}%
          </span>
        )}
      </div>

      {/* Action buttons */}
      <div className="flex space-x-2">
        {/* Select All/None toggle */}
        <button
          onClick={handleSelectAll}
          disabled={totalFiles === 0}
          className={cn(
            'flex items-center space-x-2 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200',
            'border-cyber-600/50 hover:border-neon-500/50 border',
            'bg-cyber-700/50 hover:bg-cyber-600/70',
            'text-cyber-300 hover:text-neon-300',
            'disabled:cursor-not-allowed disabled:opacity-50',
            'flex-1'
          )}
        >
          {allSelected ? <Square className="h-4 w-4" /> : <CheckSquare className="h-4 w-4" />}
          <span>{allSelected ? 'Deselect All' : 'Select All'}</span>
        </button>

        {/* Copy to clipboard */}
        <button
          onClick={handleCopyToClipboard}
          disabled={selectedCount === 0}
          className={cn(
            'flex items-center space-x-2 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200',
            'border-neon-600/50 hover:border-neon-500 border',
            'bg-neon-600/20 hover:bg-neon-600/30',
            'text-neon-300 hover:text-neon-100',
            'disabled:cursor-not-allowed disabled:opacity-50',
            'glow-border'
          )}
        >
          <Copy className="h-4 w-4" />
          <span>Copy</span>
        </button>

        {/* Download */}
        <button
          onClick={handleDownload}
          disabled={selectedCount === 0}
          className={cn(
            'flex items-center space-x-2 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200',
            'border-electric-600/50 hover:border-electric-500 border',
            'bg-electric-600/20 hover:bg-electric-600/30',
            'text-electric-300 hover:text-electric-100',
            'disabled:cursor-not-allowed disabled:opacity-50'
          )}
        >
          <Download className="h-4 w-4" />
          <span>Download</span>
        </button>
      </div>
    </div>
  )
}
