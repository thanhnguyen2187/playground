import { assign, setup } from "xstate";
import type { NoteDisplay } from "../data/schema-triplit";
import { createEmptyNoteDisplay } from "../data/data-transformation";

export const machine = setup({
  types: {
    context: {} as {
      notes: NoteDisplay[];
      note: NoteDisplay;
      searchTags: Set<string>;
    },
    events: {} as
      | { type: "Save" }
      | { type: "Check" }
      | { type: "Cancel" }
      | { type: "Failed" }
      | { type: "FailedData" }
      | { type: "Succeeded" }
      | { type: "OpenFilter" }
      | { type: "FailedAction" }
      | { type: "OpenSettings" }
      | { type: "SucceededAction" }
      | { type: "Retry" }
      | { type: "Input" }
      | { type: "Clear" }
      | { type: "Retried" }
      | { type: "Error" }
      | { type: "Loaded"; notes: NoteDisplay[] }
      | { type: "Reload" }
      | { type: "SearchTagAdd"; tag: string }
      | { type: "SearchTagRemove"; tag: string }
      | { type: "ModalOpenNote"; note: NoteDisplay }
      | { type: "ModalOpenEncryption"; note: NoteDisplay }
      | { type: "ModalConfirmDeletion" }
      | { type: "ModalCancel" },
  },
  guards: {
    IsNotesEmpty: ({ context }) => context.notes.length === 0,
  },
}).createMachine({
  context: {
    notes: [],
    note: createEmptyNoteDisplay(),
    searchTags: new Set(),
  },
  id: "AppState",
  initial: "Functioning",
  states: {
    Functioning: {
      initial: "Loading",
      on: {
        FailedData: "DataError",
      },
      states: {
        Loading: {
          on: {
            Loaded: {
              target: "Idling",
              actions: assign({
                notes: ({ event }) => {
                  console.log("loaded");
                  return event.notes;
                },
              }),
            },
            Error: "..DataError",
          },
        },
        Idling: {
          type: "parallel",
          on: {
            ModalOpenNote: {
              target: ".Modal.Note",
              actions: assign({
                note: ({ event }) => event.note,
              }),
            },
            ModalOpenEncryption: {
              target: ".Modal.Encryption",
            },
            ModalConfirmDeletion: {
              target: ".Modal.Deletion",
            },
            Reload: {
              target: "Loading",
            },
            SearchTagAdd: {
              target: "Loading",
              actions: assign({
                searchTags: ({ event, context }) => {
                  context.searchTags.add(event.tag);
                  return new Set(context.searchTags);
                },
              }),
            },
            SearchTagRemove: {
              target: "Loading",
              actions: assign({
                searchTags: ({ event, context }) => {
                  context.searchTags.delete(event.tag);
                  return new Set(context.searchTags);
                },
              }),
            },
          },
          states: {
            Items: {
              initial: "Transient",
              states: {
                Transient: {
                  always: [
                    {
                      guard: "IsNotesEmpty",
                      target: "Blank",
                    },
                    {
                      target: "Filled",
                    },
                  ],
                },
                Blank: {},
                Filled: {},
              },
            },
            Toaster: {},
            LoadingMore: {
              initial: "Idling",
              states: {
                Idling: {},
                Working: {},
              },
            },
            Modal: {
              initial: "None",
              on: {
                ModalCancel: {
                  target: ".None",
                },
              },
              states: {
                None: {},
                Encryption: {},
                Settings: {},
                Deletion: {},
                Note: {},
              },
            },
          },
        },
      },
    },
    DataError: {},
  },
});
