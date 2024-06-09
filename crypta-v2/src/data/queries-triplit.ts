import type { TriplitClient } from "@triplit/client";
import type {
	Note,
	NoteDisplay,
	NoteTag,
	NoteWithoutID,
} from "./schema-triplit";

export async function notesRead(
	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	client: TriplitClient<any>,
	limit: number,
): Promise<NoteDisplay[]> {
	const query = (
		client
		.query("notes")
		.include("tags")
		.order("createdAt", "ASC")
		.limit(limit)
		.build()
	);
	// @ts-ignore
	const result = await client.fetch(query);
	const notes = Array.from(result.values());
	const noteDisplays = notes.map((note) => ({
		...note,
		tags: (
			Array
			.from((note.tags ?? new Map()).values())
			// @ts-ignore
			.map((tag: NoteTag) => tag.tagText)
			.sort()
		),
	}));
	return noteDisplays as NoteDisplay[];
}

export async function notesUpsert(
	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	client: TriplitClient<any>,
	noteDisplay: NoteDisplay,
) {
	await client.transact(async (tx) => {
		let noteId = noteDisplay.id;
		if (noteId === "") {
			const resultInsert = await tx.insert("notes", {
				title: noteDisplay.title,
				text: noteDisplay.text,
				encrypted: noteDisplay.encrypted,
			});
			noteId = resultInsert.id;
		} else {
			await tx.insert("notes", {
				id: noteId,
				title: noteDisplay.title,
				text: noteDisplay.text,
				encrypted: noteDisplay.encrypted,
			});
		}
		const query = client
			.query("noteTags")
			.select(["id"])
			.where("noteId", "=", noteId)
			.build();
		const noteTagIdsMap = await tx.fetch(query);
		const noteTagIds = Array.from(noteTagIdsMap.keys());
		for (const noteTagId of noteTagIds) {
			await tx.delete("noteTags", noteTagId);
		}
		for (const tagText of noteDisplay.tags) {
			await tx.insert("noteTags", {
				noteId,
				tagText,
			});
		}
	});
}

export async function notesDelete(
	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	client: TriplitClient<any>,
	noteId: string,
) {
	await client.transact(async (tx) => {
		await tx.delete("notes", noteId)
		const query = (
			client
			.query("noteTags")
			.select(["id"])
			.where("noteId", "=", noteId)
			.build()
		);
		const noteTagIdsMap = await tx.fetch(query);
		const noteTagIds = Array.from(noteTagIdsMap.keys());
		for (const noteTagId of noteTagIds) {
			await tx.delete("noteTags", noteTagId);
		}
	})
}
