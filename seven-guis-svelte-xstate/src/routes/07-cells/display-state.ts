import type { Readable } from 'svelte/store'
import { derived } from 'svelte/store';
import type { Context } from './machine'
import { generateCellValues, generateColumnHeaders, generateKey, generateRowHeaders } from './machine';

export type DisplayState = {
  columnHeaders: string[]
  rowHeaders: string[]
  cellValues: string[][]
}

export function createDisplayState(contextStore: Readable<Context>): Readable<DisplayState> {
  return derived(
    contextStore,
    (context) => {
      const columnHeaders = generateColumnHeaders(context.boundaries.columnCount)
      const rowHeaders = generateRowHeaders(context.boundaries.rowCount)
      const cellValues: string[][] = generateCellValues(rowHeaders, columnHeaders, context.cellInputs)
      return {
        columnHeaders,
        rowHeaders,
        cellValues,
      }
    }
  )
}
