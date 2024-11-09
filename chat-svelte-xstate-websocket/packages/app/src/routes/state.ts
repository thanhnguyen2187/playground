import { assign, fromPromise, setup } from "xstate";

export namespace Context {
  export type Type = {
    messages: string[];
  };

  export const initial: Type = {
    messages: [
      "Hello, World!",
      "This is a chat app built with XState and Svelte.",
    ],
  };
}

export namespace Event {
  export type Increment = {
    type: "Increment";
  };

  export type Decrement = {
    type: "Decrement";
  };

  export type All = Increment | Decrement;
}

export namespace Actor {
  export const AsyncIncrement = fromPromise(
    async ({ input }: { input: number }) => {
      return input + 1;
    },
  );

  export const map = {
    asyncIncrement: AsyncIncrement,
  };
}

export const machine = setup({
  types: {
    context: {} as Context.Type,
    events: {} as Event.All,
  },
  actors: Actor.map,
}).createMachine({
  id: "Global",
  initial: "Idling",
  context: Context.initial,
  states: {
    Idling: {},
  },
  on: {
  },
});
