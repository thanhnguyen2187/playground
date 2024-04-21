import { assign, createMachine, setup } from "xstate"
import { browser } from '$app/environment'
import type { Item } from '$lib/data'
import { loadItems, saveItems } from '$lib/data'

const isMobile = browser && /iPhone|iPad|iPod|Android/i.test(window.navigator.userAgent)

export const machine = createMachine({
  types: {
    context: {} as {
      items: Item[],
    },
    events: {} as {
      type: 'Exit'
      items: Item[]
    },
  },
  context: {
    items: [],
  },
  id: 'Shakerr',
  initial: 'Transient',
  states: {
    Transient: {
      entry: assign({
        items: loadItems(),
      }),
      always: [
        {
          guard: ({}) => {
            return isMobile
          },
          target: 'Functioning',
        },
        {
          target: 'Desktop Warning',
        },
      ]
    },
    Functioning: {
      initial: 'Transient',
      on: {
        'Open Settings': {
          target: 'Settings',
        },
      },
      states: {
        'Transient': {
          always: [
            {
              guard: ({context}) => {
                return context.items.length < 2
              },
              target: 'No Data',
            },
            {
              target: 'No Result',
            },
          ],
        },
        'No Data': {},
        'No Result': {
          on: {
            Shaked: {
              target: 'Shaking',
            },
          },
        },
        Shaking: {
          on: {
            Shaked: {
              target: 'Shaking',
            },
          },
          after: {
            '500': {
              target: 'Show Result',
            },
          },
        },
        'Show Result': {
          on: {
            Reset: {
              target: 'No Result',
            },
          },
        },
        History: {
          type: 'history',
          history: 'shallow',
        },
      },
    },
    'Desktop Warning': {
      on: {
        Accept: {
          target: 'Functioning',
        },
      },
    },
    Settings: {
      initial: 'Idling',
      on: {
        Exit: {
          target: 'Functioning',
          actions: assign({
            items: ({event, context}) => {
              saveItems(event.items)
              return event.items
            },
          }),
        },
      },
      states: {
        Idling: {},
      },
    },
  },
});
