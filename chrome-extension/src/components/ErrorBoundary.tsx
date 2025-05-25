import { Component, ReactNode } from 'react'
import { AlertTriangle } from 'lucide-react'

interface ErrorBoundaryState {
  hasError: boolean
  error?: Error
}

interface ErrorBoundaryProps {
  children: ReactNode
}

export default class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props)
    this.state = { hasError: false }
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error }
  }

  componentDidCatch(error: Error, errorInfo: any) {
    console.error('Contexter Error:', error, errorInfo)
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="w-full h-full bg-gradient-to-br from-cyber-900 via-cyber-800 to-cyber-900 flex items-center justify-center p-6">
          <div className="text-center space-y-4">
            <div className="w-16 h-16 mx-auto bg-red-500/20 border border-red-500/30 rounded-full flex items-center justify-center">
              <AlertTriangle className="w-8 h-8 text-red-400" />
            </div>
            <h2 className="text-xl font-semibold text-white">Something went wrong</h2>
            <p className="text-cyber-300 text-sm max-w-xs">
              The extension encountered an unexpected error. Please try refreshing or restart the extension.
            </p>
            <button
              onClick={() => window.location.reload()}
              className="px-4 py-2 bg-red-600 hover:bg-red-500 text-white rounded-lg transition-colors"
            >
              Refresh
            </button>
          </div>
        </div>
      )
    }

    return this.props.children
  }
} 