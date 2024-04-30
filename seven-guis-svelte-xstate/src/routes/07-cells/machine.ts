import { assign, createMachine } from 'xstate'

export type TableBoundaries = {
  columnCount: number
  rowCount: number
}

export type CellInputMap = {
  [key: string]: string
}

export type Event = {
  type: 'cellInputs.set'
  key: string
  value: string
}

const columnCharacters = [
  'A', 'B', 'C', 'D', 'E', 'F',
  'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R',
  'S', 'T', 'U', 'V', 'W', 'X',
  'Y', 'Z',
]

export const defaultBoundaries: TableBoundaries = {
  columnCount: columnCharacters.length,
  rowCount: 30,
}

export const defaultCellInputs: CellInputMap = {
  '2,4': 'abc',
}

export type Context = {
  boundaries: TableBoundaries
  cellInputs: CellInputMap
}

export const cellMachine = createMachine({
  id: '0,0',
  context: {
    value: '',
  },
})

export const cellsMachine = createMachine({
  id: 'cells',
  types: {} as {
    context: Context
    events: Event
  },
  context: {
    boundaries: defaultBoundaries,
    cellInputs: defaultCellInputs,
  },
  on: {
    'cellInputs.set': {
      actions: assign({
        cellInputs: ({context, event}) => ({
          ...context.cellInputs,
          [event.key]: event.value,
        })
      }),
    }
  },
})

export function generateColumnHeaders(columnCount: number) {
  if (columnCount === columnCharacters.length) {
    return columnCharacters
  }
  throw Error(`generateRowHeaders: unsupported columnCount ${columnCount}`)
}

export function generateRowHeaders(rowCount: number) {
  return Array.from(
    Array(rowCount),
    (_, index) => index + 1
  ).map(
    index => index.toString()
  )
}

export function generateKey(rowIndex: number, columnIndex: number) {
  return `${rowIndex},${columnIndex}`
}

export function findCellIndex(label: string): [number, number] {
  const [columnHeader, rowHeader] = [label.slice(0,1), label.slice(1,)]
  const rowIndex = Number(rowHeader) - 1
  const columnIndex = columnCharacters.indexOf(columnHeader)
  return [rowIndex, columnIndex]
}

export function generateCellValues(
  rowHeaders: string[],
  columnHeaders: string[],
  contextValues: CellInputMap,
) {
  const cellValues: string[][] = []
  for (let i = 0; i < rowHeaders.length; i++) {
    cellValues.push([])
    for (let j = 0; j < columnHeaders.length; j++) {
      const key = generateKey(i, j)
      if (contextValues[key]) {
        cellValues[i].push(contextValues[key])
      } else {
        cellValues[i].push('')
      }
    }
  }
  return cellValues
}
