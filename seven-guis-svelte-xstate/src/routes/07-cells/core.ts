export type CellState =
  | 'plain'
  | 'valid-formula'
  | 'invalid-formula'

export type Cell = {
  rowIndex: number
  columnIndex: number
  value: string
}

export type CellWithState =
  Cell
  & {
    state: CellState
  }

export type Core = {
  input(cell: Cell): void
  output(): CellWithState[]
}

export function createCore(): Core {
  const cellsRaw = new Map<number, Map<number, string>>()
  const cellsTree = []

  return {
    input(cell: Cell) {
    },
    output(): CellWithState[] {
      return []
    },
  }
}
