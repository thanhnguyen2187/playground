import {
	sqliteTable,
	text,
	integer,
	real,
	unique,
	primaryKey,
} from "drizzle-orm/sqlite-core";
import { sql } from "drizzle-orm";

export const notes = sqliteTable("notes", {
	id: text("id").primaryKey(),
	name: text("name").notNull(),
	language: text("language").notNull(),
	text: text("text").notNull(),
	encrypted: integer("encrypted", { mode: "boolean" }).default(false).notNull(),
	position: real("position").default(1).notNull(),
	updatedAt: text("updated_at").default(sql`CURRENT_TIMESTAMP`).notNull(),
	createdAt: text("created_at").default(sql`CURRENT_TIMESTAMP`).notNull(),
});

export const note_tags = sqliteTable(
	"note_tags",
	{
		noteId: text("note_id")
			.references(() => notes.id, {
				onDelete: "cascade",
				onUpdate: "cascade",
			})
			.notNull(),
		tagText: text("tag_text").notNull(),
		updatedAt: text("updated_at").default(sql`CURRENT_TIMESTAMP`).notNull(),
		createdAt: text("created_at").default(sql`CURRENT_TIMESTAMP`).notNull(),
	},
	(t) => ({
		pk: primaryKey({
			name: "primary_key__note_id__tag_text",
			columns: [t.noteId, t.tagText],
		}),
	}),
);
