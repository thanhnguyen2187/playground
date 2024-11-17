import type { Actor, AnyStateMachine, StateFrom } from "xstate";

/**
 * Add a convenience `state` property to an XState actor. The property is a
 * Svelte rune, which means it has reactive updates in a Svelte component.
 * */
export function wrap<T extends AnyStateMachine>(actor: Actor<T>) {
  let state = $state.raw<StateFrom<T>>(actor.getSnapshot());
  actor.subscribe((s) => {
    state = s;
  });
  actor.start();
  return {
    get state() {
      return state;
    },
    ...actor,
  };
}
