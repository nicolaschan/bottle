const WebSocket = require('ws');
 
const wss = new WebSocket.Server({
  port: 8080
});

wss.on('connection', ws => {
  console.log('connection')
  ws.on('message', data => {
    console.log(data)
    wss.clients.forEach(client => {
      if (client !== ws && client.readyState === WebSocket.OPEN) {
        client.send(data)
      }
    })
  })
})