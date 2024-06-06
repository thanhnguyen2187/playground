import type { NoteDisplay, NoteWithoutID } from './schema-triplit';

export function createEmptyNoteDisplay(): NoteDisplay {
  return {
    id: "",
    title: "To be filled",
    text: "To be filled",
    tags: [
      "tag-1", "tag-2"
    ],
    createdAt: new Date(),
    updatedAt: new Date(),
  }
}

export function transform_NoteDisplay_NoteWithoutID(
  noteDisplay: NoteDisplay,
): NoteWithoutID {
  return {
    ...noteDisplay,
    encrypted: false,
  }
}

export function transform_NoteDisplayTags_NoteTagWithoutIDs(noteId: string, tags: string[]) {
  return tags.map(
    tag => {
      return {
        noteId,
        textTag: tag,
      }
    }
  )
}
