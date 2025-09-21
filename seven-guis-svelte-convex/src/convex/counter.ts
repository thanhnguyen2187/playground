import { mutation, query } from "./_generated/server";

export const get = query({
  args: {},
  handler: async (ctx) => {
    const record = await ctx.db.query("counter").first();
    return record?.value ?? 0;
  },
});

export const increase = mutation({
  args: {},
  handler: async (ctx) => {
    const record = await ctx.db.query("counter").first();
    if (record === null) {
      await ctx.db.insert("counter", {
        value: 1,
      });
    } else {
      record.value += 1;
      await ctx.db.replace(record._id, record);
    }
  },
});
