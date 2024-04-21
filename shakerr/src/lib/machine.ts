import { assign, createMachine, setup } from "xstate"
import { browser } from '$app/environment'
import type { Item } from '$lib/data'
import { loadItems, saveItems } from '$lib/data'

const isMobile = browser && /iPhone|iPad|iPod|Android/i.test(window.navigator.userAgent)

export const machine = createMachine({
  types: {
    context: {} as {
      items: Item[],
      pickedIndex: number,
    },
    events: {} as {
      type: 'Exit'
      items: Item[]
    },
  },
  context: {
    items: [],
    pickedIndex: -1,
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
              actions: assign({
                pickedIndex: ({context}) => {
                  return Math.floor(Math.random() * context.items.length)
                },
              }),
              target: 'Shaking',
            },
          },
        },
        Shaking: {
          on: {
            Shaked: {
              actions: assign({
                pickedIndex: ({context}) => {
                  return Math.floor(Math.random() * context.items.length)
                },
              }),
              target: 'Shaking',
              reenter: true,
            },
          },
          after: {
            '1500': {
              target: 'Show Result',
            },
          },
        },
        'Show Result': {
          on: {
            Shaked: {
              actions: assign({
                pickedIndex: ({context}) => {
                  return Math.floor(Math.random() * context.items.length)
                },
              }),
              target: 'Shaking',
            },
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
          target: 'Functioning.History',
          actions: assign({
            items: ({event}) => {
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
