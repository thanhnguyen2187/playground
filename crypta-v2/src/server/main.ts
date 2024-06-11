import { createServer } from "@triplit/server";
import { schema } from '../data/schema-triplit';

const port = +(process.env.PORT || 8080);

const startServer = createServer({
  storage: "memory",
  verboseLogs: true,
  dbOptions: {
  },
});

const dbServer = startServer(port);

console.log("running on port", port);
process.on("SIGINT", () => {
  dbServer.close(() => {
    console.log("Shutting down server... ");
    process.exit();
  });
});
