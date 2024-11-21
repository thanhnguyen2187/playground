import { expect, test } from "vitest";
import { AppSchema } from "./powersync";
import { PowerSyncDatabase } from "@powersync/web";

test("Create todo", async () => {
  const db = new PowerSyncDatabase({
    schema: AppSchema,
    database: {
      dbFilename: ":memory:",
    },
  });
  const todo = {
    name: "Buy milk",
    done: false,
  };
  const result = await db.execute(
    "INSERT INTO todos(name, done) VALUES (?, ?)",
    [todo.name, todo.done],
  );

  expect(result.insertId).not.toBeUndefined;
});
