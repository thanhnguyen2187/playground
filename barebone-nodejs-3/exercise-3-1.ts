/**
 * You are tasked with implementing a logging mechanism in NodeJS for monitoring
 * transaction processing in real time. This logging mechanism should support
 * three different logging levels: info, warning, and error. The implementation
 * should also provide a way to aggregate logs, making it straightforward to
 * visualize the logs for monitoring purposes.
 *
 * The task can be broken down into the following steps:
 *
 * - Create a simple NodeJS transaction processing system with a function that
 *   simulates processing a transaction.
 * - Implement the logging mechanism supporting 'info', 'warning', and 'error'
 *   levels.
 * - Modify the transaction processing function to generate logs at appropriate
 *   levels during the transaction lifecycle.
 * - Provide a way to aggregate these logs in a structured format that would
 *   facilitate easy visualization.
 *
 * Sample transaction object:
 *
 * ```
 * {
 *   "transactionId": "12345",
 *   "amount": 100.00,
 *   "status": "pending"
 * }
 * ```
 *
 * Example log format:
 *
 * ```
 * {
 *   "timestamp": "2023-10-31T12:00:00Z",
 *   "level": "info",
 *   "message": "Transaction 12345 started, amount: 100.00"
 * }
 * ```
 * */

import * as http from 'http'

export type Log = {
  level: 'info' | 'warning' | 'error'
  timestamp: number
  message: string
}

export type Logger = {
  info(message: string): void
  warning(message: string): void
  error(message: string): void

  getAll(): Log[]
  clear(): void
}

export function createLog(level: 'info' | 'warning' | 'error', message: string): Log {
  return {
    level,
    timestamp: Date.now(),
    message,
  }
}

export function createLogger(): Logger {
  const logs: Log[] = []

  return {
    info(message: string) {
      const log = createLog('info', message)
      logs.push(log)
      console.info(log)
    },
    warning(message: string) {
      const log = createLog('warning', message)
      logs.push(log)
      console.info(log)
    },
    error(message: string) {
      const log = createLog('error', message)
      logs.push(log)
      console.info(log)
    },
    getAll(): Log[] {
      return logs
    },
    clear(): void {
      // might need to revisit this
      logs.length = 0
    }
  }
}

const globalLogger = createLogger()

export type Transaction = {
  transactionId: string,
  amount: number,
  status: string,
}

const server = http.createServer((req, res) => {
  if (req.method === 'POST' && req.url === '/process') {
    let bodyStr = ''
    req.on('data', (chunk) => {
      bodyStr += chunk.toString()
    })
    req.on('end', () => {
      try {
        const bodyTyped = JSON.parse(bodyStr) as Transaction
        globalLogger.info(`received transaction ${bodyTyped.transactionId}`)
        if (bodyTyped.status !== 'pending') {
          globalLogger.warning(`transaction ${bodyTyped.transactionId} is already in progress`)
          res.writeHead(400) // indicate input's error
          res.end(JSON.stringify({success: false}))
          return
        }
        // stimulate handling the transaction
        globalLogger.info(`done processing ${bodyTyped.transactionId}`)

        res.writeHead(200)
        res.end(JSON.stringify({success: true}))
      } catch (error) {
        res.writeHead(500) // indicate server's error
        globalLogger.error(`error happened ${error.toString()}`)
      }
    })
  } else if (req.method === 'GET' && req.url === '/logs') {
    res.writeHead(200)
    res.end(JSON.stringify({logs: globalLogger.getAll()}))
  } else if (req.method === 'POST' && req.url === '/logs/clear') {
    res.writeHead(200)
    globalLogger.clear()
    globalLogger.info(`cleared logs`)
  } else {
    res.writeHead(404)
  }
})

server.listen(3000)

/**
 * Example CURL commands:
 *
 * ```
 * curl -X POST \
 *     --data-raw '
 * {
 *     "transactionId": "12345",
 *     "amount": 100.00,
 *     "status": "pending"
 * }' \
 *     127.0.0.1:3000/process
 * ```
 *
 * Output:
 *
 * ```
 * {
 *   level: 'info',
 *   timestamp: 1732079244717,
 *   message: 'received transaction 12345'
 * }
 * {
 *   level: 'info',
 *   timestamp: 1732079244719,
 *   message: 'done processing 12345'
 * }
 * ```
 * */
