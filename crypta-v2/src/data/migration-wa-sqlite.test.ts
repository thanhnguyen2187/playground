import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { setupServer } from "msw/node";
import { createDb, createSQLiteAPI, createQueryExecutor } from "./wa-sqlite";
import type { WASQLiteExecutor } from "./wa-sqlite";
import type { SqliteRemoteDatabase } from "drizzle-orm/sqlite-proxy";
import { createWASqliteMockWASMHandler } from "./testing";
import { migrate } from "./migration-wa-sqlite";
import {
	defaultMigrationQueryMap,
	defaultQueriesStringMap,
} from "./migration-common";
import { sql } from 'drizzle-orm';

const handlers = createWASqliteMockWASMHandler();
const server = setupServer(...handlers);
beforeAll(() => {
	server.listen({ onUnhandledRequest: "error" });
});
afterAll(() => {
	server.close();
});

describe("happy path", async () => {
	let sqliteAPI: SQLiteAPI;
	let executor: WASQLiteExecutor;
	let localDb: SqliteRemoteDatabase;

	beforeAll(async () => {
		sqliteAPI = await createSQLiteAPI("http://mock.local", "MemoryAsyncVFS");
		executor = await createQueryExecutor(sqliteAPI, "crypta", false);
		localDb = createDb(executor);
	});

	it("migrate", async () => {
		await migrate(localDb, defaultMigrationQueryMap, defaultQueriesStringMap)
		{
			const result = await localDb.run(sql`PRAGMA user_version`)
			expect(result).toEqual({rows: [[1]]})
		}
		{
			const result = await localDb.run(sql`SELECT * FROM notes`)
			expect(result).toEqual({rows: []})
		}
	});
});
