import { RefreshCw, Settings, Zap } from 'lucide-react'
import { cn } from '../lib/utils'

interface HeaderProps {
  onRefresh: () => void
}

export default function Header({ onRefresh }: HeaderProps) {
  return (
    <header className="glass-dark border-b border-neon-500/20 p-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <div className="pulse-dot flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-r from-neon-500 to-electric-600">
            <Zap className="h-5 w-5 text-white" />
          </div>
          <h1 className="gradient-text font-mono text-xl font-bold tracking-tight">Contexter</h1>
        </div>

        <div className="flex items-center space-x-2">
          <button
            onClick={onRefresh}
            className={cn(
              'rounded-lg p-2 transition-all duration-200',
              'bg-cyber-700/50 hover:bg-cyber-600/70',
              'border border-cyber-500/30 hover:border-neon-500/50',
              'text-cyber-300 hover:text-neon-400',
              'glow-border'
            )}
            title="Refresh Projects"
          >
            <RefreshCw className="h-4 w-4" />
          </button>

          <button
            onClick={() => chrome.runtime.openOptionsPage()}
            className={cn(
              'rounded-lg p-2 transition-all duration-200',
              'bg-cyber-700/50 hover:bg-cyber-600/70',
              'border border-cyber-500/30 hover:border-neon-500/50',
              'text-cyber-300 hover:text-neon-400',
              'glow-border'
            )}
            title="Settings"
          >
            <Settings className="h-4 w-4" />
          </button>
        </div>
      </div>
    </header>
  )
}
