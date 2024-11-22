import { expect, test } from "vitest";
import { createStore } from "tinybase";
import { randomUUID } from "node:crypto";

test("Create store", async () => {
  const store = createStore();
  expect(store.getTables()).toEqual({});
});

test("CRUD todo", async () => {
  const store = createStore();
  store.setTablesSchema({
    todos: {
      name: { type: "string" },
      done: { type: "boolean", default: false },
    },
  });

  const id = randomUUID();
  store.setRow("todos", id, { name: "test" });
  const row = store.getRow("todos", id);

  expect(row).toEqual({
    name: "test",
    done: false,
  });

  store.getRowCount("todos")
});
