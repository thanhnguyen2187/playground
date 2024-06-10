import type { TriplitClient } from "@triplit/client";
import { or } from "@triplit/db";
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
  tags: Set<string>,
): Promise<NoteDisplay[]> {
  let noteDisplays: NoteDisplay[] = [];
  await client.transact(async (tx) => {
    let query = client
      .query("notes")
      .include("tags")
      .order("createdAt", "ASC")
      .limit(limit);
    if (tags.size > 0) {
      let noteIds = [];
      {
        const query = client
          .query("noteTags")
          .select(["noteId"])
          // @ts-ignore
          .where("tagText", "in", new Set(tags))
          .build();
        const result = await client.fetch(query);
        noteIds = Array.from(result.values()).map(({noteId}) => noteId);
      }
      // const whereClause = [or(noteIds.map((noteId) => ["id", "=", noteId]))];
      const whereClause = [["id", "in", noteIds]];
      query = query
        // @ts-ignore
        .where(whereClause);
    }
    // @ts-ignore
    const result = await client.fetch(query.build());
    const notes = Array.from(result.values());
    noteDisplays = notes.map((note) => ({
      ...note,
      tags: Array.from((note.tags ?? new Map()).values())
        // @ts-ignore
        .map((tag: NoteTag) => tag.tagText)
        .sort(),
    }));
  });
  return noteDisplays;
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
    await tx.delete("notes", noteId);
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
  });
}
