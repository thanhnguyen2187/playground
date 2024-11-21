import { Schema as S } from "@triplit/client";
import { TriplitClient, type ClientSchema } from "@triplit/client";

export const schema = {
  todos: {
    schema: S.Schema({
      id: S.Id(),
      name: S.String(),
      done: S.Boolean(),
    }),
  },
} satisfies ClientSchema;
