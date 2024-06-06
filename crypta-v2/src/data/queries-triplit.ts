import type { TriplitClient } from "@triplit/client";
import type { Note, NoteDisplay, NoteTag, NoteWithoutID } from './schema-triplit';

export async function notesRead(
	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	client: TriplitClient<any>,
	limit: number,
): Promise<NoteDisplay[]> {
	const query = client.query("notes").limit(limit).build();
	const result = await client.fetch(query);
	const notes = Array.from(result.values());
	const noteDisplays = notes.map(
		(note) => ({
			...note,
			tags: (note.tags ?? []).map((tag: NoteTag) => tag.textTag),
		})
	)
	return noteDisplays as NoteDisplay[];
}

export async function notesUpsert(
	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	client: TriplitClient<any>,
	noteDisplay: NoteDisplay,
) {
	const note: Note | NoteWithoutID = {
		...noteDisplay,
		encrypted: false,
		tags: [],
	};
	await client.insert("notes", {
		title: note.title,
		text: note.text,
		encrypted: false,
	})
}
