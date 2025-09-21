import { v } from "convex/values";
import { mutation, query } from "./_generated/server";

export const get = query({
  args: {},
  handler: async (ctx) => {
    const record = await ctx.db.query("temperature_converter").first();
    return record ? [record.valueCelsius, record.valueFahrenheit] : [0, 32];
  },
});

export function convert(
  value: number,
  from: "celsius" | "fahrenheit",
  to: "celsius" | "fahrenheit",
) {
  switch (true) {
    case from === "celsius" && to === "fahrenheit":
      return value * 1.8 + 32;
    case from === "fahrenheit" && to === "celsius":
      return (value - 32) / 1.8;
    default:
      return value;
  }
}

export const setValueCelsius = mutation({
  args: { value: v.number() },
  handler: async (ctx, args) => {
    const record = await ctx.db.query("temperature_converter").first();
    const valueFahrenheit = convert(args.value, "celsius", "fahrenheit");
    if (record === null) {
      await ctx.db.insert("temperature_converter", {
        valueCelsius: args.value,
        valueFahrenheit,
      });
    } else {
      await ctx.db.replace(record._id, {
        valueCelsius: args.value,
        valueFahrenheit,
      });
    }
  },
});

export const setValueFahrenheit = mutation({
  args: { value: v.number() },
  handler: async (ctx, args) => {
    const record = await ctx.db.query("temperature_converter").first();
    const valueCelsius = convert(args.value, "fahrenheit", "celsius");
    if (record === null) {
      await ctx.db.insert("temperature_converter", {
        valueCelsius,
        valueFahrenheit: args.value,
      });
    } else {
      await ctx.db.replace(record._id, {
        valueCelsius,
        valueFahrenheit: args.value,
      });
    }
  },
});
