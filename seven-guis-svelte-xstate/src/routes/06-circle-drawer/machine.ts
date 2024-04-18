import { assign, createMachine } from 'xstate'

export type Context = {
  renderFn: (circles: Circle[], selectedIndex: number) => void
  circles: Circle[]
  selectedIndex: number
  selectedRadius: number
  history: Circle[][]
  historyIndex: number
}

export type Input = {
  renderFn: (circles: Circle[], selectedIndex: number) => void
}

export type Circle = {
  x: number
  y: number
  r: number
}

export type Event =
  {
    type: 'circle.add'
    circle: Circle,
  } |
  {
    type: 'circle.select'
    x: number
    y: number
  } |
  {
    type: 'circle.edit'
    index: number
  } |
  {
    type: 'circle.save'
    r: number
  } |
  {
    type: 'circle.mutate'
    r: number
  } |
  {
    type: 'circle.cancel'
  } |
  {
    type: 'undo'
  } |
  {
    type: 'redo'
  }


export function square(x: number): number {
  return x * x
}


export function calculateDistance(x1: number, y1: number, x2: number, y2: number): number {
  return Math.sqrt(square(x2 - x1) + square(y2 - y1))
}


export function saveCircle(circles: Circle[], i: number, r: number): Circle[] {
  return [
    ...circles.slice(0, i),
    {...circles[i], r},
    ...circles.slice(i + 1),
  ]
}

export function addHistoryEntry(history: Circle[][], historyIndex: number, circles: Circle[]): Circle[][] {
  return [
    ...history.slice(0, historyIndex + 1),
    circles,
  ]
}


export const circleDrawerMachine = createMachine({
  id: 'circle-drawer',
  types: {} as {
    context: Context,
    input: Input,
  },
  context: ({input}) => ({
    renderFn: input.renderFn,
    circles: [],
    selectedIndex: -1,
    selectedRadius: 0,
    history: [[]],
    historyIndex: 0,
  }),
  initial: 'idling',
  states: {
    'idling': {
      on: {
        'circle.select': {
          actions: ({context, event, self}) => {
            for (let i = 0; i < context.circles.length; i++) {
              const circle = context.circles[i]
              if (calculateDistance(circle.x, circle.y, event.x, event.y) <= circle.r) {
                self.send({
                  type: 'circle.edit',
                  index: i,
                })
                return
              }
            }
            self.send({
              type: 'circle.add',
              circle: {
                x: event.x,
                y: event.y,
                r: 10,
              }
            })
          },
        },
        'circle.add': {
          actions: [
            assign({
              circles: ({context, event}) => [...context.circles, event.circle],
              renderFn: ({context, event}: {context: Context, event: any}) => {
                context.circles.push(event.circle)
                context.renderFn(context.circles, context.selectedIndex)
                return context.renderFn
              },
              history: ({context, event}) =>
                addHistoryEntry(
                  context.history,
                  context.historyIndex,
                  [...context.circles, event.circle],
                ),
              historyIndex: ({context}) => context.historyIndex + 1,
            })
          ],
        },
        'circle.edit': {
          target: 'editing',
          actions: assign({
            selectedIndex: ({event}) => event.index,
            selectedRadius: ({context, event}) => context.circles[event.index].r,
            // @ts-ignore
            renderFn: ({context, event}) => {
              context.renderFn(context.circles, event.index)
              return context.renderFn
            },
          }),
        },
        'undo': {
          guard: ({context}) => context.historyIndex > 0,
          actions: assign({
            selectedIndex: ({}) => -1,
            // @ts-ignore
            renderFn: ({context, event}) => {
              const circles = context.history[context.historyIndex - 1]
              context.renderFn(circles, -1)
              return context.renderFn
            },
            circles: ({context}) => context.history[context.historyIndex - 1].slice(),
            historyIndex: ({context}) => context.historyIndex - 1,
          }),
        },
        'redo': {
          guard: ({context}) => context.historyIndex < context.history.length - 1,
          actions: assign({
            selectedIndex: ({}) => -1,
            // @ts-ignore
            renderFn: ({context}) => {
              const circles = context.history[context.historyIndex + 1]
              context.renderFn(circles, -1)
              return context.renderFn
            },
            circles: ({context}) => context.history[context.historyIndex + 1].slice(),
            historyIndex: ({context}) => context.historyIndex + 1,
          }),
        },
      }
    },
    'editing': {
      on: {
        'circle.select': {
          actions: ({context, event, self}) => {
            const circle = context.circles[context.selectedIndex]
            if (
              calculateDistance(circle.x, circle.y, event.x, event.y) > context.selectedRadius ||
              context.selectedRadius === circle.r
            ) {
              self.send({
                type: 'circle.cancel',
              })
            } else {
              self.send({
                type: 'circle.save',
              })
            }
          },
        },
        'circle.save': {
          target: 'idling',
          actions: assign({
            circles: ({context}) => {
              return saveCircle(context.circles, context.selectedIndex, context.selectedRadius)
            },
            selectedIndex: -1,
            // @ts-ignore
            renderFn: ({context}) => {
              context.renderFn(saveCircle(context.circles, context.selectedIndex, context.selectedRadius), -1)
              return context.renderFn
            },
            historyIndex: ({context}) => context.historyIndex + 1,
            history: ({context, event}) =>
              addHistoryEntry(
                context.history,
                context.historyIndex,
                saveCircle(
                  context.circles,
                  context.selectedIndex,
                  context.selectedRadius,
                ),
              ),
          })
        },
        'circle.mutate': {
          actions: assign({
            selectedRadius: ({event}) => event.r,
            // @ts-ignore
            renderFn: ({context, event}) => {
              context.renderFn(saveCircle(context.circles, context.selectedIndex, context.selectedRadius), context.selectedIndex)
              return context.renderFn
            },
          }),
        },
        'circle.cancel': {
          target: 'idling',
          actions: assign({
            selectedIndex: -1,
            // @ts-ignore
            renderFn: ({context}) => {
              context.renderFn(context.circles, -1)
              return context.renderFn
            },
          }),
        },
      },
    },
  },
  on: {
  },
})
