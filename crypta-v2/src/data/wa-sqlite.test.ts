import { describe, it, expect, beforeAll, afterAll } from 'vitest'
import { setupServer } from 'msw/node'
import {
  createDb,
  createSQLiteAPI,
  createQueryExecutor,
} from './wa-sqlite'
import type { WASQLiteExecutor } from './wa-sqlite'
import type { SqliteRemoteDatabase } from 'drizzle-orm/sqlite-proxy'
import { createWASqliteMockWASMHandler } from './testing'
import { sql } from 'drizzle-orm'

const handlers = createWASqliteMockWASMHandler()
const server = setupServer(...handlers)
beforeAll(() => {
  server.listen({onUnhandledRequest: 'error'})
})
afterAll(() => {
  server.close()
})

describe('happy path', async () => {

  // it('basic query', async () => {
  //   expect(1 + 1).toBe(2)
  // })

  let sqliteAPI: SQLiteAPI
  let executor: WASQLiteExecutor
  let localDb: SqliteRemoteDatabase

  beforeAll(async () => {
    sqliteAPI = await createSQLiteAPI('http://mock.local', 'MemoryAsyncVFS')
    executor = await createQueryExecutor(sqliteAPI, 'crypta', false)
    localDb = createDb(executor)
  })

  it('basic query', async () => {
    {
      const result = await executor.execute('SELECT 1')
      expect(result).toEqual([[1]])
    }
    {
      const result = await executor.execute('PRAGMA user_version')
      expect(result).toEqual([[0]])
    }
  })

  it('local db', async () => {
    {
      const result = await localDb.run(sql`SELECT 1`)
      expect(result).toEqual({rows: [[1]]})
    }
    {
      const result = await localDb.run(sql`PRAGMA user_version`)
      expect(result).toEqual({rows: [[0]]})
    }
  })

})
