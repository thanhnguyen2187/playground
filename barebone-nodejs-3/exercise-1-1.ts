/**
 * The service should connect to a message queue (preferably RabbitMQ) named
 * transaction_queue (or a queue of your choice if using a different system).
 * Transactions should be processed asynchronously. High-value transactions
 * (value > 1000) should be prioritized over low-value transactions. This means
 * that the service should ensure high-value transactions are processed as soon
 * as possible, even if they arrive after low-value transactions. Once a
 * transaction is processed, log the transaction details to the console.
 *
 * Sample data:
 *
 * ```
 * {
 *   "id": "txn_1",
 *   "value": 500,
 *   "timestamp": "2023-10-01T10:00:00Z"
 * }
 * {
 *   "id": "txn_2",
 *   "value": 1500,
 *   "timestamp": "2023-10-01T10:02:00Z"
 * }
 * {
 *   "id": "txn_3",
 *   "value": 300,
 *   "timestamp": "2023-10-01T10:03:00Z"
 * }
 * ```
 * */

import { Connection } from 'rabbitmq-client'
import { MaxPriorityQueue } from '@datastructures-js/priority-queue'

type Transaction = {
  id: string
  value: number
  timestamp: string
}

// Store the transactions in a priority queue to ensure that higher value
// transactions are processed first.
const transactions = new MaxPriorityQueue<Transaction>((tx) => tx.value)
const rabbit = new Connection('amqp://sample:sample@localhost:5672')
rabbit.on('error', (err) => {
  console.error('rabbit: RabbitMQ connection error', err)
})
rabbit.on('connection', () => {
  console.info('rabbit: Connection successfully (re)established')
})

// Stimulate a function that keeps on watching the priority queue and process
async function handleMessage() {
  const tx = transactions.dequeue()
  // ...
  console.info('process: Processed transaction with id ', tx.id)
}

const sub = rabbit.createConsumer({
  queue: 'transaction_queue',
  queueOptions: {durable: true},
  // handle 2 messages at a time
  qos: {prefetchCount: undefined, prefetchSize: 0},
}, async (msg) => {
  const tx: Transaction = JSON.parse(msg.body)
  transactions.enqueue(tx)
})

sub.on('error', (err) => {
  console.error('consumer: consumer error (user-events)', err)
})

async function onShutdown() {
  await sub.close()
  await rabbit.close()
}

process.on('SIGINT', onShutdown)
process.on('SIGTERM', onShutdown)
