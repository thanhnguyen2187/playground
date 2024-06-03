import { Schema as S } from "@triplit/client";
import type { ClientSchema, Entity } from "@triplit/client";

export const schema = {
	notes: {
		schema: S.Schema({
			id: S.Id(),
			title: S.String(),
			text: S.String(),
			encrypted: S.Boolean(),
			tags: S.RelationMany("noteTags", {
				where: [["noteId", "=", "$id"]],
			}),
			updatedAt: S.Date({ default: S.Default.now() }),
			createdAt: S.Date({ default: S.Default.now() }),
		}),
	},
	noteTags: {
		schema: S.Schema({
			id: S.Id(),
			noteId: S.String(),
			tagText: S.String(),
		}),
	},
} satisfies ClientSchema;

export type Note = Entity<typeof schema, 'notes'>
export type NoteTag = Entity<typeof schema, 'noteTags'>
