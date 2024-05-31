import { createMachine } from 'xstate'

export const machine = createMachine({
  id: 'machine',
  initial: 'view',
  states: {
    view: {
      on: {
        CLICK: 'active'
      }
    },
  }
})
