import { useExtensionStore } from '../lib/store'
import { Zap } from 'lucide-react'

export default function Settings() {
  const { settings, updateSettings } = useExtensionStore()

  return (
    <div className="min-h-screen p-8">
      <div className="mx-auto max-w-2xl">
        <div className="glass-dark border-neon-500/20 rounded-xl border p-8">
          <div className="mb-8 flex items-center space-x-3">
            <div className="from-neon-500 to-electric-600 flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-r">
              <Zap className="h-6 w-6 text-white" />
            </div>
            <h1 className="gradient-text font-mono text-2xl font-bold">Contexter Settings</h1>
          </div>

          <div className="space-y-6">
            <div>
              <label className="text-cyber-300 mb-2 block text-sm font-medium">API Key</label>
              <input
                type="password"
                value={settings.apiKey}
                onChange={(e) => updateSettings({ apiKey: e.target.value })}
                className="border-cyber-600/30 bg-cyber-800/50 placeholder-cyber-400 focus:border-neon-500/50 w-full rounded-lg border px-4 py-3 text-white transition-colors focus:outline-none"
                placeholder="Enter your API key"
              />
            </div>

            <div>
              <label className="text-cyber-300 mb-2 block text-sm font-medium">Server URL</label>
              <input
                type="text"
                value={settings.serverUrl}
                onChange={(e) => updateSettings({ serverUrl: e.target.value })}
                className="border-cyber-600/30 bg-cyber-800/50 placeholder-cyber-400 focus:border-neon-500/50 w-full rounded-lg border px-4 py-3 text-white transition-colors focus:outline-none"
                placeholder="http://localhost:3030"
              />
            </div>

            <div>
              <label className="text-cyber-300 mb-2 block text-sm font-medium">Theme</label>
              <select
                value={settings.theme}
                onChange={(e) =>
                  updateSettings({ theme: e.target.value as 'system' | 'light' | 'dark' })
                }
                className="border-cyber-600/30 bg-cyber-800/50 focus:border-neon-500/50 w-full rounded-lg border px-4 py-3 text-white transition-colors focus:outline-none"
              >
                <option value="system">Follow System</option>
                <option value="light">Light Mode</option>
                <option value="dark">Dark Mode</option>
              </select>
            </div>
          </div>

          <div className="border-cyber-600/30 mt-8 border-t pt-6">
            <p className="text-cyber-400 text-sm">
              Configure your Contexter server connection and preferences.
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
