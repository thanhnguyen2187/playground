import * as http from 'http'
import * as fs from 'fs'
import * as zlib from 'zlib'
import * as stream from 'stream'

const filePathSource = '/tmp/file.txt'
const filePathDestination = '/tmp/file-compressed.gz'

const server = http.createServer((req, res) => {
  res.setHeader('Content-Type', 'application/json')
  if (req.method === 'POST' && req.url === '/compress') {
    const streamRead = fs.createReadStream(filePathSource)
    const streamWrite = fs.createWriteStream(filePathDestination)

    streamRead.on('error', (error) => {
      console.log('Error reading file: ', error)
      res.writeHead(500)
      res.end(JSON.stringify({message: 'Error reading file'}))
    })

    streamWrite.on('error', (error) => {
      console.log('Error reading file: ', error)
      res.writeHead(500)
      res.end(JSON.stringify({message: 'Error writing file'}))
    })

    const pipeline = stream.pipeline(
      streamRead,
      zlib.createGzip(),
      streamWrite,
      (error) => {
        if (error) {
          console.log('Error compressing file: ', error)
          res.writeHead(500)
          res.end(JSON.stringify({message: 'Error compressing file: '}))
          return
        }

        res.writeHead(200)
        res.end(JSON.stringify({message: 'Success'}))
      },
    )
  } else {
    res.writeHead(404)
  }
})

server.listen(3000)
