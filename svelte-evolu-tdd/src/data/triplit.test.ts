import { expect, test } from "vitest";
import { TriplitClient } from "@triplit/client";
import { schema } from "./triplit";

test("Create todo", async () => {
  const client = new TriplitClient({ schema, storage: "memory" });
  const todo = {
    name: "Buy milk",
    done: false,
  };

  const { txId, output } = await client.insert("todos", todo);

  expect(txId).not.toBeUndefined;
});
