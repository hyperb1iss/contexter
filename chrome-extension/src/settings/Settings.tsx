import { useExtensionStore } from '../lib/store'
import { Zap } from 'lucide-react'

export default function Settings() {
  const { settings, updateSettings } = useExtensionStore()

  return (
    <div className="min-h-screen p-8">
      <div className="max-w-2xl mx-auto">
        <div className="glass-dark p-8 rounded-xl border border-neon-500/20">
          <div className="flex items-center space-x-3 mb-8">
            <div className="w-10 h-10 bg-gradient-to-r from-neon-500 to-electric-600 rounded-lg flex items-center justify-center">
              <Zap className="w-6 h-6 text-white" />
            </div>
            <h1 className="text-2xl font-bold gradient-text font-mono">
              Contexter Settings
            </h1>
          </div>

          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-cyber-300 mb-2">
                API Key
              </label>
              <input
                type="password"
                value={settings.apiKey}
                onChange={(e) => updateSettings({ apiKey: e.target.value })}
                className="w-full px-4 py-3 bg-cyber-800/50 border border-cyber-600/30 rounded-lg text-white placeholder-cyber-400 focus:border-neon-500/50 focus:outline-none transition-colors"
                placeholder="Enter your API key"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-cyber-300 mb-2">
                Server URL
              </label>
              <input
                type="text"
                value={settings.serverUrl}
                onChange={(e) => updateSettings({ serverUrl: e.target.value })}
                className="w-full px-4 py-3 bg-cyber-800/50 border border-cyber-600/30 rounded-lg text-white placeholder-cyber-400 focus:border-neon-500/50 focus:outline-none transition-colors"
                placeholder="http://localhost:3030"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-cyber-300 mb-2">
                Theme
              </label>
              <select
                value={settings.theme}
                onChange={(e) => updateSettings({ theme: e.target.value as 'system' | 'light' | 'dark' })}
                className="w-full px-4 py-3 bg-cyber-800/50 border border-cyber-600/30 rounded-lg text-white focus:border-neon-500/50 focus:outline-none transition-colors"
              >
                <option value="system">Follow System</option>
                <option value="light">Light Mode</option>
                <option value="dark">Dark Mode</option>
              </select>
            </div>
          </div>

          <div className="mt-8 pt-6 border-t border-cyber-600/30">
            <p className="text-cyber-400 text-sm">
              Configure your Contexter server connection and preferences.
            </p>
          </div>
        </div>
      </div>
    </div>
  )
} 