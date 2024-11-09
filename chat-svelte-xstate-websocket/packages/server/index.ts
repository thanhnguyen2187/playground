import uWS from "uWebSockets.js";

uWS
  .App()
  .ws("/*", {
    open: (ws) => {
      ws.send("Hello world!");
    },
    message: (ws, message, isBinary) => {
      ws.send(message);
    },
    close: () => {},
  })
  .listen(8080, () => {});
