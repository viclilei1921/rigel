import type { Metric } from 'web-vitals'

import { onCLS, onFCP, onINP, onLCP, onTTFB } from 'web-vitals'

import { logger } from './LoggerWrapper'

type WebVitalMetric
  = | (Metric & { type: 'web-vital' })
    | {
      type: 'long-task'
      name: string
      startTime: number
      duration: number
      attribution?: Record<string, unknown>
    }

type Reporter = (metric: WebVitalMetric) => void

function defaultReporter(metric: WebVitalMetric) {
  const label = metric.type === 'web-vital' ? metric.name : 'long-task'
  logger.info('[performance]', label, metric)
}

let hasStarted = false

export function startWebVitalObserver(reporter: Reporter = defaultReporter) {
  if (hasStarted || typeof window === 'undefined') {
    return
  }
  hasStarted = true

  const report = (metric: Metric) => {
    reporter({
      ...metric,
      type: 'web-vital'
    })
  }

  onCLS(report, { reportAllChanges: true })
  onFCP(report)
  onINP(report, { reportAllChanges: true })
  onLCP(report, { reportAllChanges: true })
  onTTFB(report)

  if (
    'PerformanceObserver' in window
    && Array.isArray((PerformanceObserver as any).supportedEntryTypes)
    && (PerformanceObserver as any).supportedEntryTypes.includes('long-task')
  ) {
    const observer = new PerformanceObserver((entryList) => {
      for (const entry of entryList.getEntries()) {
        reporter({
          type: 'long-task',
          name: entry.name || 'long-task',
          startTime: entry.startTime,
          duration: entry.duration,
          attribution: (entry as any).attribution
        })
      }
    })

    try {
      observer.observe({ type: 'long-task', buffered: true })
    } catch (error) {
      console.warn('[performance] long-task observer failed:', error)
    }
  }
}
