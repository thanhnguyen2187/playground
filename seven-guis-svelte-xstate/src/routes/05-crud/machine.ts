import { assign, createMachine, raise } from 'xstate'

export type Record = {
  name: string
  surname: string
}

export type DisplayRecord =
  Record &
  {
    index: number
  }

export type Event =
  {
    type: 'filter.set',
    value: string,
  } |
  {
    type: 'record.select',
    index: number,
  } |
  {
    type: 'record.update',
  } |
  {
    type: 'record.mutate',
    key: keyof Record,
    value: any,
  } |
  {
    type: 'record.add',
  } |
  {
    type: 'record.delete',
  }

export const defaultRecords = [
  {
    surname: 'Emil',
    name: 'Hans',
  },
  {
    surname: 'Mustermann',
    name: 'Max',
  },
  {
    surname: 'Tisch',
    name: 'Roman',
  },
]

export function addIndex(records: Record[]): DisplayRecord[] {
  return records.map((record, index) => {
    return {
      ...record,
      index,
    }
  })
}

export function filterRecords(records: Record[], filter: string): DisplayRecord[] {
  const filteredRecords = addIndex(records).filter(
    record =>
      record.surname.toLowerCase().includes(filter.toLowerCase()) ||
      record.name.toLowerCase().includes(filter.toLocaleLowerCase())
  )
  return filteredRecords
}

export const crudMachine = createMachine({
  id: 'crud',
  types: {} as {
    context: {
      filter: string
      records: Record[],
      displayRecords: DisplayRecord[],
      selectedIndex: number,
      selectedRecord: Record,
      updateEnabled: boolean,
      deleteEnabled: boolean,
    },
    events: Event,
  },
  context: {
    filter: '',
    records: defaultRecords,
    displayRecords: addIndex(defaultRecords),
    selectedIndex: -1,
    selectedRecord: {
      surname: '',
      name: '',
    },
    updateEnabled: true,
    deleteEnabled: true,
  },
  on: {
    'filter.set': {
      actions: assign({
        filter: ({event}) => event.value,
        displayRecords: ({context, event}) => filterRecords(context.records, event.value)
      }),
    },
    'record.select': {
      actions: assign({
        selectedIndex: ({context, event}) => {
          if (context.selectedIndex === event.index) {
            return -1
          }
          return event.index
        },
        selectedRecord: ({context, event}) => {
          if (context.selectedIndex === event.index || event.index === -1) {
            return {
              surname: '',
              name: '',
            }
          }
          return {...context.records[event.index]}
        },
      }),
    },
    'record.mutate': {
      guard: ({context}) => context.selectedIndex !== -1,
      actions: assign({
        selectedRecord: ({context, event}) => {
          return {
            ...context.selectedRecord,
            [event.key]: event.value,
          }
        },
      }),
    },
    'record.update': {
      guard: ({context}) => context.selectedIndex !== -1,
      actions: assign({
        records: ({context}) => {
          context.records[context.selectedIndex] = context.selectedRecord
          return [...context.records]
        },
        displayRecords: ({context}) => {
          context.records[context.selectedIndex] = context.selectedRecord
          return filterRecords(context.records, context.filter)
        }
      }),
    },
    'record.add': {
      actions: [
        assign({
          records: ({context, event}) => {
            return [
              ...context.records,
              context.selectedRecord,
            ]
          },
          displayRecords: ({context, event}) => {
            return filterRecords(
              [...context.records, context.selectedRecord],
              context.filter,
            )
          },
        }),
        raise({type: 'record.select', index: -1}),
      ],
    },
    'record.delete': {
      guard: ({context}) => context.selectedIndex !== -1,
      actions: [
        assign({
          records: ({context}) => {
            context.records.splice(context.selectedIndex, 1)
            return [
              ...context.records
            ]
          },
          displayRecords: ({context}) => {
            context.displayRecords.splice(context.selectedIndex, 1)
            return [
              ...context.displayRecords
            ]
          },
        }),
        raise({type: 'record.select', index: -1}),
      ],
    },
  },
})
