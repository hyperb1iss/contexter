import { useExtensionStore } from '../lib/store'
import { Zap } from 'lucide-react'

export default function Settings() {
  const { settings, updateSettings } = useExtensionStore()

  return (
    <div className="min-h-screen p-8">
      <div className="mx-auto max-w-2xl">
        <div className="glass-dark rounded-xl border border-neon-500/20 p-8">
          <div className="mb-8 flex items-center space-x-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-r from-neon-500 to-electric-600">
              <Zap className="h-6 w-6 text-white" />
            </div>
            <h1 className="gradient-text font-mono text-2xl font-bold">Contexter Settings</h1>
          </div>

          <div className="space-y-6">
            <div>
              <label className="mb-2 block text-sm font-medium text-cyber-300">API Key</label>
              <input
                type="password"
                value={settings.apiKey}
                onChange={(e) => updateSettings({ apiKey: e.target.value })}
                className="w-full rounded-lg border border-cyber-600/30 bg-cyber-800/50 px-4 py-3 text-white placeholder-cyber-400 transition-colors focus:border-neon-500/50 focus:outline-none"
                placeholder="Enter your API key"
              />
            </div>

            <div>
              <label className="mb-2 block text-sm font-medium text-cyber-300">Server URL</label>
              <input
                type="text"
                value={settings.serverUrl}
                onChange={(e) => updateSettings({ serverUrl: e.target.value })}
                className="w-full rounded-lg border border-cyber-600/30 bg-cyber-800/50 px-4 py-3 text-white placeholder-cyber-400 transition-colors focus:border-neon-500/50 focus:outline-none"
                placeholder="http://localhost:3030"
              />
            </div>

            <div>
              <label className="mb-2 block text-sm font-medium text-cyber-300">Theme</label>
              <select
                value={settings.theme}
                onChange={(e) =>
                  updateSettings({ theme: e.target.value as 'system' | 'light' | 'dark' })
                }
                className="w-full rounded-lg border border-cyber-600/30 bg-cyber-800/50 px-4 py-3 text-white transition-colors focus:border-neon-500/50 focus:outline-none"
              >
                <option value="system">Follow System</option>
                <option value="light">Light Mode</option>
                <option value="dark">Dark Mode</option>
              </select>
            </div>
          </div>

          <div className="mt-8 border-t border-cyber-600/30 pt-6">
            <p className="text-sm text-cyber-400">
              Configure your Contexter server connection and preferences.
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
