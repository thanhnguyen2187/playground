import { Schema } from "@effect/schema";
import { id, table, database, SqliteBoolean } from "@evolu/common";
import { createEvolu } from "@evolu/common-web";

const TodoId = id("Todo");
type TodoId = typeof TodoId.Type;

export const TodoTable = table({
  id: TodoId,
  name: Schema.String,
  done: SqliteBoolean,
});
export type TTodoTable = typeof TodoTable.Type;

export const Database = database({
  todo: TodoTable,
});
export type TDatabase = typeof Database.Type;

export const evolu = createEvolu(Database);
