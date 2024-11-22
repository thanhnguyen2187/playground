import { Dexie, type EntityTable } from "dexie";

export interface Todo {
  id: string;
  name: string;
  done: boolean;
}

export function createDb() {
  const db = new Dexie("todos") as Dexie & {
    todos: EntityTable<Todo, "id">;
  };
  db.version(1).stores({
    todos: "++id, name, done",
  });
  return db;
}
