import type { Config } from "drizzle-kit";

export default {
	schema: "./src/data/schema.ts",
	driver: "turso",
	out: "./db",
	dialect: "sqlite",
} satisfies Config;
