import { match, P } from 'ts-pattern'

export type CellType =
  | 'plain'
  | 'valid-formula'
  | 'invalid-formula'

export type CellCoordinates = {
  rowIndex: number
  columnIndex: number
}

export type Cell =
  CellCoordinates
  & {
    value: string
  }

export type CellTyped =
  Cell
  & {
    type: CellType
  }

export type ParseResult = |
  {
    type: 'valid-formula'
    dependencies: CellCoordinates[]
  } |
  {
    type: 'invalid-formula'
  } |
  {
    type: 'plain'
  }

export type ParseNode = {}
export type ParseTree = {}

export type EvaluateResult = {
}

export type Core = {
  input(cell: Cell): void
  output(): CellTyped[]
}

export function generateKey(rowIndex: number, columnIndex: number) {
  return `${rowIndex},${columnIndex}`
}

export function parseCellValue(value: string): ParseResult {
  if (value.startsWith('=')) {
    const tokens = value.slice(1).split(' ').filter(token => token.length > 0)
    const stack = []
    for (const token of tokens) {
      if (['+', '-', '/', '*'].includes(token)) {

      }
    }
  }

  return {
    type: 'plain',
  }
}

export function parseTokens(tokens: string[]): ParseResult {
  return {
    type: 'plain',
  }
}

export function createCore(): Core {
  const cellsRaw: {[key: string]: string} = {}
  const cellsParsed: {[key: string]: CellTyped} = {}
  const cellTrees = []

  return {
    input(cell: Cell) {
      const key = generateKey(cell.rowIndex, cell.columnIndex)
    },
    output(): CellTyped[] {
      return []
    },
  }
}
