import { useExtensionStore } from '../lib/store'
import { Loader2 } from 'lucide-react'

export default function LoadingOverlay() {
  const { loadingMessage } = useExtensionStore()

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="glass-dark p-6 rounded-xl border border-neon-500/30 text-center space-y-4">
        <div className="flex justify-center">
          <Loader2 className="w-8 h-8 text-neon-400 animate-spin" />
        </div>
        <p className="text-white font-medium">
          {loadingMessage || 'Loading...'}
        </p>
        <div className="flex justify-center space-x-1">
          <div className="w-2 h-2 bg-neon-500 rounded-full animate-pulse" style={{ animationDelay: '0ms' }} />
          <div className="w-2 h-2 bg-neon-500 rounded-full animate-pulse" style={{ animationDelay: '150ms' }} />
          <div className="w-2 h-2 bg-neon-500 rounded-full animate-pulse" style={{ animationDelay: '300ms' }} />
        </div>
      </div>
    </div>
  )
} 