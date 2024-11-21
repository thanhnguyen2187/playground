import { column, Schema, Table } from "@powersync/web";

const todos = new Table({
  name: column.text,
  done: column.integer,
});

export const AppSchema = new Schema({
  todos,
});

export type Database = (typeof AppSchema)["types"];
export type TodoRecord = Database["todos"];
