import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  counter: defineTable({
    value: v.number(),
  }),
  temperature_converter: defineTable({
    valueCelsius: v.number(),
    valueFahrenheit: v.number(),
  }),
});
