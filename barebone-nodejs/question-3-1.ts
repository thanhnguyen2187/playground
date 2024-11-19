import * as http from 'http'
import * as url from 'node:url';

// map from endpoint to array of response times
const responseTimesMap = new Map<string, number[]>();

type HTTPResponse = http.ServerResponse<http.IncomingMessage> & {
  req: http.IncomingMessage
}

function generateId(req: http.IncomingMessage) {
  return `${req.method} ${req.url}`
}

function average(numbers: number[]): string {
  let sum = 0
  for (const number of numbers) {
    sum += number
  }
  return (sum / numbers.length).toFixed(2)
}

function addMiddlewareTimer(
  resp: HTTPResponse,
  responseTimesMap: Map<string, number[]>,
) {
  const startMs = Date.now()
  resp.on('finish', () => {
    const endMs = Date.now()
    const elapsedMs = endMs - startMs
    if (elapsedMs >= 200) {
      console.warn(`Request ${resp.req.method} ${resp.req.url} took ${elapsedMs}ms`)
    }
    const id = generateId(resp.req)
    const responseTimes = responseTimesMap.get(id)
    if (responseTimes) {
      responseTimes.push(elapsedMs)
    } else {
      responseTimesMap.set(id, [elapsedMs])
    }
  })
}

const server = http.createServer((req, resp) => {
  resp.setHeader('Content-Type', 'application/json')
  addMiddlewareTimer(resp, responseTimesMap)

  if (req.method === 'GET' && req.url === '/fast') {
    resp.end(JSON.stringify({message: 'fast'}))
  } else if (req.method === 'GET' && req.url === '/slow') {
    resp.writeHead(200)
    setTimeout(
      () => resp.end(JSON.stringify({message: 'slow'})),
      500,
    )
  } else if (req.method === 'GET' && req.url === '/random') {
    resp.writeHead(200)
    setTimeout(
      () => resp.end(JSON.stringify({message: 'random'})),
      Math.random() * 500,
    )
  } else if (req.method === 'GET' && req.url === '/report') {
    resp.writeHead(200)
    const result = Array.from(responseTimesMap.entries()).map(
      ([key, value]) => ({endpoint: key, averageMs: average(value)})
    )
    resp.end(JSON.stringify(result))
  } else {
    resp.writeHead(404)
  }
})

server.listen(3000)
