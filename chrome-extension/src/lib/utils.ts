import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

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
      return '🔷'
    case 'java':
      return '☕'
    case 'md':
      return '📝'
    case 'json':
      return '📄'
    case 'toml':
    case 'yaml':
    case 'yml':
      return '⚙️'
    case 'css':
    case 'scss':
    case 'sass':
      return '🎨'
    case 'html':
      return '🌐'
    default:
      return '📄'
  }
}

export function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text
  return text.slice(0, maxLength) + '...'
}
