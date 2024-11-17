import { assign, fromPromise, setup } from "xstate";
import { convert, type Temperature } from "./logics";

export namespace Context {
  export type Type = {
    currentC: Temperature;
    currentF: Temperature;
  };

  export const initial: Type = {
    currentC: {
      value: 0,
      unit: "C",
    },
    currentF: convert(0, "C", "F"),
  };
}

export namespace Event {
  export type InputC = {
    type: "InputC";
    value: number;
  };
  export type InputF = {
    type: "InputF";
    value: number;
  };

  export type All = InputC | InputF;
}

export namespace Actor {
  export const map = {};
}

export const machine = setup({
  types: {
    context: {} as Context.Type,
    events: {} as Event.All,
  },
  actors: Actor.map,
}).createMachine({
  id: "TemperatureConverter",
  initial: "Idling",
  context: Context.initial,
  states: {
    Idling: {},
    Error: {},
  },
  on: {
    InputC: {
      actions: assign(({ context, event }) => {
        const newC: Temperature = {
          value: event.value,
          unit: "C",
        };
        const newF = convert(event.value, "C", "F");
        return {
          currentC: newC,
          currentF: newF,
        };
      }),
    },
    InputF: {
      actions: assign(({ context, event }) => {
        const newF: Temperature = {
          value: event.value,
          unit: "F",
        };
        const newC = convert(event.value, "F", "C");
        return {
          currentC: newC,
          currentF: newF,
        };
      }),
    },
  },
});
