import { assign, createMachine, fromCallback } from 'xstate'

export const timerMachine = createMachine({
  id: 'timer',
  types: {} as {
    context: {
      duration: number,
      elapsed: number,
    },
    events: {
      type: 'duration.set',
      value: number,
    } | {
      type: 'start' | 'tick' | 'stop' | 'reset',
    },
  },
  context: {
    duration: 5_000,
    elapsed: 0,
  },
  invoke: {
    src: fromCallback(({ sendBack }) => {
      const interval = setInterval(
        () => sendBack({type: 'tick'}),
        100,
      )
      return () => clearInterval(interval)
    }),
  },
  initial: 'idling',
  states: {
    idling: {
      on: {
        start: {
          target: 'running',
          actions: assign({
            elapsed: 0,
          }),
        }
      },
    },
    running: {
      on: {
        tick: {
          actions: assign({
            elapsed: ({context, self}) => {
              const newElapsed = context.elapsed + 100
              if (newElapsed <= context.duration) {
                return newElapsed
              }
              self.send({type: 'stop'})
              return context.duration
            },
          }),
        },
        stop: {
          target: 'stopped',
        }
      }
    },
    stopped: {},
  },
  on: {
    'duration.set': {
      target: '.running',
      actions: assign({
        duration: ({event}) => event.value,
      }),
    },
    'reset': {
      target: '.running',
      actions: assign({
        elapsed: () => 0,
      }),
    },
  },
})
