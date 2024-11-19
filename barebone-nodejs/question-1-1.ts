import * as http from 'http'

const store = {}

const server = http.createServer((req, res) => {
  if (req.method === 'POST' && req.url === '/register') {
    let bodyStr = ''
    req.on('data', (chunk) => {
      bodyStr += chunk
    })
    req.on('end', () => {
      res.setHeader('Content-Type', 'application/json')
      let body
      try {
        body = JSON.parse(bodyStr)
      } catch (err) {
        res.writeHead(400)
        console.error(err)
        res.end(JSON.stringify({message: 'invalid body format'}))
        return
      }

      if (
        !('username' in body) ||
        !('password' in body) ||
        !('email' in body)
      ) {
        res.writeHead(400)
        res.end(JSON.stringify({message: 'invalid body format; expected username, password, and email'}))
        return
      }

      const isValidUsername = body.username.match(/[a-zA-Z0-9]+/) !== null
      if (!isValidUsername) {
        res.writeHead(400)
        res.end(JSON.stringify({message: 'invalid username; expected alphanumeric characters only'}))
        return
      }

      const isValidPassword = body.password.length >= 8
      if (!isValidPassword) {
        res.writeHead(400)
        res.end(JSON.stringify({message: 'invalid password; expected at least 8 characters'}))
        return
      }

      const isValidEmail = body.email.match(/^[a-zA-Z0-9_]+@[a-zA-Z0-9_]+$/) !== null
      if (!isValidEmail) {
        res.writeHead(400)
        res.end(JSON.stringify({message: 'invalid email; expected valid email address'}))
        return
      }

      store[body.username] = {...body}
      console.log('stored user and sent email')
      res.writeHead(200)
      res.end(JSON.stringify({message: 'success'}))
    })

    return
  }

  res.writeHead(404)
})

server.listen(3001)
