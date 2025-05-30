import { useExtensionStore } from '../lib/store'
import { Loader2 } from 'lucide-react'

export default function LoadingOverlay() {
  const { loadingMessage } = useExtensionStore()

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="glass-dark border-neon-500/30 space-y-4 rounded-xl border p-6 text-center">
        <div className="flex justify-center">
          <Loader2 className="text-neon-400 h-8 w-8 animate-spin" />
        </div>
        <p className="font-medium text-white">{loadingMessage || 'Loading...'}</p>
        <div className="flex justify-center space-x-1">
          <div
            className="bg-neon-500 h-2 w-2 animate-pulse rounded-full"
            style={{ animationDelay: '0ms' }}
          />
          <div
            className="bg-neon-500 h-2 w-2 animate-pulse rounded-full"
            style={{ animationDelay: '150ms' }}
          />
          <div
            className="bg-neon-500 h-2 w-2 animate-pulse rounded-full"
            style={{ animationDelay: '300ms' }}
          />
        </div>
      </div>
    </div>
  )
}
