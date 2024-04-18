import { assign, createMachine, raise } from 'xstate'

export function convert(value: number, from: 'C' | 'F', to: 'C' | 'F'): number {
  if (from === 'C' && to === 'F') {
    return (value * 9) / 5 + 32
  } else if (from === 'F' && to === 'C') {
    return ((value - 32) * 5) / 9
  }
  throw Error('convert: unreachable code')
}

export const temperatureConverterMachine = createMachine({
  id: 'temperatureConverter',
  types: {} as {
    context: {
      valueCelsius: number
      valueFahrenheit: number
    },
    events: {
      type: 'valueCelsius.set',
      valueCelsius: number,
    } | {
      type: 'valueFahrenheit.set',
      valueFahrenheit: number,
    },
  },
  context: {
    valueCelsius: 0,
    valueFahrenheit: convert(0, 'C', 'F'),
  },
  on: {
    'valueCelsius.set': {
      actions: assign({
        valueCelsius: ({event}) => event.valueCelsius,
        valueFahrenheit: ({event, context}) => {
          if (isNaN(event.valueCelsius)) {
            return context.valueFahrenheit
          }
          return convert(event.valueCelsius, 'C', 'F')
        }
      }),
    },
    'valueFahrenheit.set': {
      actions: assign({
        valueFahrenheit: ({event}) => event.valueFahrenheit,
        valueCelsius: ({event, context}) => {
          if (isNaN(event.valueFahrenheit)) {
            return context.valueCelsius
          }
          return convert(event.valueFahrenheit, 'F', 'C')
        }
      }),
    },
  },
})
