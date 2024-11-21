/**
 * Implement a Node.js application that demonstrates your understanding of
 * concurrency and the event loop using worker threads. Specifically, you will
 * create a server that handles multiple requests concurrently by offloading
 * CPU-intensive tasks to worker threads.
 *
 * Requirements
 *
 * 1. Create an HTTP server using the http module that listens on port 3000.
 * 2. When a request is received on the endpoint /compute, the server should
 *    offload a CPU-intensive computation (for example, calculating the factorial
 *    of a large number) to a worker thread.
 * 3. The computation task should be handled by a separate script (worker file)
 *    and the result should be sent back to the server to be included in the
 *    HTTP response.
 * 4. Ensure proper handling of multiple concurrent requests and prevent
 *    blocking of the event loop.
 * */

import * as http from 'http'
import { Worker } from 'worker_threads'

const server = http.createServer((req, res) => {
  if (req.method === 'POST' && req.url === '/compute') {
    const worker = new Worker('./worker.js', {})
    worker.on('exit', (msg) => {
      res.end('DONE')
    })
  } else {
    res.writeHead(404)
  }
})

server.listen(3000)
