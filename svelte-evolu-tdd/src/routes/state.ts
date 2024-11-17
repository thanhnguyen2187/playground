import { assign, setup } from "xstate";

export namespace Constant {
  export const MachineID = "TodoMVP";
}

export namespace State {
  export const Idling = "Idling";
  export const AsyncIncreasing = "AsyncIncreasing";

  export type Type = typeof Idling | typeof AsyncIncreasing;

  export function generate(states: Type[]): string {
    return `#${Constant.MachineID}.${states.join(".")}`;
  }
}

export namespace Context {
  export type Task = {
    id: string;
    name: string;
    done: boolean;
  };

  export type Type = {
    tasksMap: Map<string, Task>;
  };

  export const initial: Type = {
    tasksMap: new Map(),
  };
}

export namespace Event {
  export type Add = {
    type: "Add";
    id: string;
    name: string;
  };

  export type Check = {
    type: "Check";
    id: string;
  };

  export type Uncheck = {
    type: "Uncheck";
    id: string;
  };

  export type Remove = {
    type: "Remove";
    id: string;
  };

  export type All = Add | Check | Uncheck | Remove;
}

export namespace Actor {
  export const map = {};
}

export namespace Guard {}

export namespace Action {}

export const machine = setup({
  types: {
    context: {} as Context.Type,
    events: {} as Event.All,
  },
  actors: Actor.map,
}).createMachine({
  id: Constant.MachineID,
  initial: State.Idling,
  context: Context.initial,
  states: {
    [State.Idling]: {},
  },
  on: {
    Add: {
      actions: assign({
        tasksMap: ({ context, event }) => {
          if (context.tasksMap.has(event.id)) {
            return context.tasksMap;
          }

          const tasksMap = context.tasksMap;
          tasksMap.set(event.id, {
            id: event.id,
            name: event.name,
            done: false,
          });
          return tasksMap;
        },
      }),
    },
    Check: {
      actions: assign({
        tasksMap: ({ context, event }) => {
          if (!context.tasksMap.has(event.id)) {
            return context.tasksMap;
          }

          const tasksMap = context.tasksMap;
          tasksMap.set(event.id, {
            ...(tasksMap.get(event.id) as Context.Task),
            done: true,
          });
          return tasksMap;
        },
      }),
    },
    Uncheck: {
      actions: assign({
        tasksMap: ({ context, event }) => {
          if (!context.tasksMap.has(event.id)) {
            return context.tasksMap;
          }

          const tasksMap = context.tasksMap;
          tasksMap.set(event.id, {
            ...(tasksMap.get(event.id) as Context.Task),
            done: false,
          });
          return tasksMap;
        },
      }),
    },
    Remove: {
      actions: assign({
        tasksMap: ({ context, event }) => {
          if (!context.tasksMap.has(event.id)) {
            return context.tasksMap;
          }

          const tasksMap = context.tasksMap;
          tasksMap.delete(event.id);
          return tasksMap;
        },
      }),
    },
  },
});
