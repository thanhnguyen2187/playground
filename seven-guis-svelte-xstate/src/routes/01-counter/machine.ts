import { assign, createMachine } from 'xstate'

export const counterMachine = createMachine({
  id: 'counter',
  types: {} as {
    context: {
      count: number
    }
  },
  context: {
    count: 0,
  },
  on: {
    'counter.increase': {
      actions: assign({
        count: ({context}) => context.count + 1,
      }),
    }
  },
})
