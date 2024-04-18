import { assign, createMachine } from 'xstate'

export function isValidDate(date: string): boolean {
  return !!date.match(/^\d{2}\.\d{2}\.\d{4}$/)
}

export function shouldEnableBooking(flightType: string, fromDate: string, toDate: string): boolean {
  if (flightType === 'oneway') {
    return isValidDate(fromDate)
  } else if (flightType === 'return') {
    return isValidDate(fromDate) && isValidDate(toDate) && fromDate <= toDate
  }
  throw Error('shouldEnableBooking: unreachable code')
}

export const flightBookerMachine = createMachine({
  id: 'flightBooker',
  types: {} as {
    context: {
      flightType: 'oneway' | 'return',
      fromDate: string,
      fromDateWarning: boolean,
      toDate: string,
      toDateWarning: boolean,
      toDateEnabled: boolean,
      bookingEnabled: boolean,
    },
    events: {
      type: 'flightType.set',
      value: 'oneway' | 'return',
    } | {
      type: 'fromDate.set',
      value: string,
    } | {
      type: 'toDate.set',
      value: string,
    }
  },
  context: {
    flightType: 'oneway',
    fromDate: '27.03.2014',
    fromDateWarning: false,
    toDate: '27.03.2014',
    toDateWarning: false,
    toDateEnabled: false,
    bookingEnabled: true,
  },
  on: {
    'flightType.set': {
      actions: assign({
        flightType: ({event}) => event.value,
        toDateEnabled: ({event}) => event.value === 'return',
        toDateWarning: ({event, context}) => event.value === 'return' && !isValidDate(context.toDate),
        bookingEnabled: ({event, context}) => shouldEnableBooking(event.value, context.fromDate, context.toDate)
      }),
    },
    'fromDate.set': {
      actions: assign({
        fromDate: ({event}) => event.value,
        fromDateWarning: ({event}) => !isValidDate(event.value),
        bookingEnabled: ({event, context}) => shouldEnableBooking(context.flightType, event.value, context.toDate)
      }),
    },
    'toDate.set': {
      actions: assign({
        toDate: ({event}) => event.value,
        toDateWarning: ({event}) => !isValidDate(event.value),
        bookingEnabled: ({event, context}) => shouldEnableBooking(context.flightType, context.fromDate, event.value)
      }),
    },
  },
})
