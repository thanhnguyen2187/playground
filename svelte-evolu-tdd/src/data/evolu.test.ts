import { expect, test, beforeEach } from "vitest";
import { Database } from "./evolu";
import { createEvolu } from "@evolu/common-web";

test("Create todo", async () => {
  const evolu = createEvolu(Database);
  const todo = {
    id: "0",
    name: "Buy milk",
    done: false,
  };

  evolu.create("todo", todo);

  const result = await evolu.loadQuery(
    evolu.createQuery((db) => db.selectFrom("todo").select("id")),
  );
  expect(result.rows).toHaveLength(1);
  // expect(result[0]).toEqual(todo);
});

// test("Read todo", async () => {
//   const todo = {
//     id: TodoId.create(),
//     title: "Buy milk",
//     isCompleted: false,
//   };
//
//   await evolu.mutate((db) => {
//     db.todo.add(todo);
//   });
//
//   const result = await evolu.query((db) => db.todo.get(todo.id));
//   expect(result).toEqual(todo);
// });
//
// test("Update todo", async () => {
//   const todo = {
//     id: TodoId.create(),
//     title: "Buy milk",
//     isCompleted: false,
//   };
//
//   await evolu.mutate((db) => {
//     db.todo.add(todo);
//   });
//
//   const updatedTodo = { ...todo, isCompleted: true };
//
//   await evolu.mutate((db) => {
//     db.todo.update(updatedTodo);
//   });
//
//   const result = await evolu.query((db) => db.todo.get(todo.id));
//   expect(result).toEqual(updatedTodo);
// });
//
// test("Delete todo", async () => {
//   const todo = {
//     id: TodoId.create(),
//     title: "Buy milk",
//     isCompleted: false,
//   };
//
//   await evolu.mutate((db) => {
//     db.todo.add(todo);
//   });
//
//   await evolu.mutate((db) => {
//     db.todo.delete(todo.id);
//   });
//
//   const result = await evolu.query((db) => db.todo.get(todo.id));
//   expect(result).toBeUndefined();
// });
