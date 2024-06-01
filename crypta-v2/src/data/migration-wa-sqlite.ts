import type { SqliteRemoteDatabase } from "drizzle-orm/sqlite-proxy";
import type { MigrationQueryMap, QueriesStringMap } from "./migration-common";
import { sql } from "drizzle-orm";

export async function migrate(
	db: SqliteRemoteDatabase,
	migrationQueryMap: MigrationQueryMap,
	queriesStringMap: QueriesStringMap,
) {
	let [currentUserVersion] = await db.get<[number]>(sql`PRAGMA user_version`);
	while (migrationQueryMap[currentUserVersion]) {
		const migrationQueryPath = migrationQueryMap[currentUserVersion];
		const migrationQueryString = queriesStringMap[migrationQueryPath];
		if (!migrationQueryString) {
			console.error(
				`migrate: could not find query string of ${migrationQueryPath}`,
			);
			return;
		}
		await db.run(sql.raw(migrationQueryString));
		// TODO: investigate why using `?` within the query doesn't work
		currentUserVersion += 1;
		await db.run(sql.raw(`PRAGMA user_version = ${currentUserVersion}`));
	}
}
