import { setup } from "xstate";
import type { Note } from '../data/schema-triplit';

export const machine = setup({
  types: {
    context: {} as {
      notes: Note[],
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
      | { type: "Loaded", items: Note[] },
  },
  guards: {},
}).createMachine({
  context: {
    notes: [],
  },
  id: "AppState",
  initial: "Functioning",
  states: {
    Functioning: {
      type: "parallel",
      on: {
        FailedData: "DataError",
      },
      states: {
        Loading: {
          on: {
            Loaded: "Idling",
            Error: "DataError",
          }
        },
        Idling: {
          states: {
            Main: {
              type: "parallel",
              states: {
                Items: {},
                Toaster: {},
                LoadingMore: {
                  states: {
                    Idling: {},
                    Working: {},
                  }
                },
              }
            },
            Modal: {
              states: {
                Transient: {},
              }
            },
          }
        },
      },
    },
    DataError: {},
  },
});
