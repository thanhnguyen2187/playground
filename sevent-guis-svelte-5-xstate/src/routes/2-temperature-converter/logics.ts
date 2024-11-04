export type Temperature = {
  value: number;
  unit: "C" | "F";
};

export function convert(
  value: number,
  from: Temperature["unit"],
  to: Temperature["unit"],
): Temperature {
  if (from === to) {
    return {
      value,
      unit: to,
    };
  }
  if (from === "C" && to === "F") {
    return {
      value: (value * 9) / 5 + 32,
      unit: "F",
    };
  }
  if (from === "F" && to === "C") {
    return {
      value: ((value - 32) * 5) / 9,
      unit: "C",
    };
  }
  throw new Error(
    JSON.stringify({ message: "Unreachable code", value, from, to }),
  );
}
