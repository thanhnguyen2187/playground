import { expect, test } from "vitest";
import { TriplitClient } from "@triplit/client";
import { schema } from "./triplit";
import "fake-indexeddb/auto";
import { createDb } from "./dexie";
import { randomUUID } from "node:crypto";

test("Create todo", async () => {
  const db = createDb();
  const id = randomUUID();
  const todo = {
    id,
    name: "test",
    done: false,
  };

  db.todos.add(todo);
  expect(await db.todos.get(id)).toEqual(todo);
});
