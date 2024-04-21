import { assign, createMachine, setup } from "xstate"
import { browser } from '$app/environment'
import type { Item } from '$lib/data'
import { loadItems, saveItems } from '$lib/data'

const isMobile = browser && /iPhone|iPad|iPod|Android/i.test(window.navigator.userAgent)

type Event = {
  type: 'Exit'
  items: Item[]
} | {
  type: 'Exit'
}

export const machine = createMachine({
  types: {
    context: {} as {
      items: Item[]
      pickedIndex: number
      shakingUnitMs: number
      shakingBuiltMs: number
      shakingThresholdMs: number
    },
    events: {} as {
      type: 'Exit'
      items: Item[]
    },
  },
  context: {
    items: [],
    pickedIndex: -1,
    shakingBuiltMs: 0,
    shakingUnitMs: 1000,
    shakingThresholdMs: 3000,
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
          always: [
            {
              guard: ({context}) => {
                return context.shakingBuiltMs >= context.shakingThresholdMs
              },
              target: 'Show Result',
            },
          ],
          on: {
            Shaked: {
              actions: assign({
                shakingBuiltMs: () => 0,
              }),
              reenter: true,
            },
          },
          after: {
            500: {
              actions: assign({
                shakingBuiltMs: ({context}) => context.shakingBuiltMs + context.shakingUnitMs,
                pickedIndex: ({context}) => Math.floor(Math.random() * context.items.length),
              }),
              target: 'Shaking',
              reenter: true,
            },
          }
        },
        'Show Result': {
          on: {
            Shaked: {
              actions: assign({
                shakingBuiltMs: () => 0,
              }),
              target: 'Shaking',
            },
            Reset: {
              actions: assign({
                shakingBuiltMs: () => 0,
                pickedIndex: -1,
              }),
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
