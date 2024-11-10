import uWS from "uWebSockets.js";

// Set to store all connected WebSocket clients
const clients = new Set<uWS.WebSocket<unknown>>();

uWS
  .App()
  .ws("/*", {
    // When a new client connects
    open: (ws) => {
      // Add the client to our set
      clients.add(ws);
      ws.send(
        JSON.stringify({
          type: "OPEN",
        }),
      );
    },

    // When a message is received
    message: (ws, message, isBinary) => {
      // Broadcast to all clients except sender
      for (const client of clients) {
        client.send(
          JSON.stringify({
            type: "MESSAGE",
            value: Buffer.from(message).toString("utf8").trim(),
          }),
          isBinary,
        );
      }
    },

    // When a client disconnects
    close: (ws) => {
      // Remove the client from our set
      clients.delete(ws);
    },
  })
  .listen(8080, (listenSocket) => {
    if (listenSocket) {
      console.log("Listening on port 8080");
    }
  });
