<script lang="ts">
import { machine } from "./state";
import { createActor } from "xstate";
import { wrap } from "$lib/xstate-wrap.svelte.js";

const actor = wrap(createActor(machine));

function handleClick() {
  actor.ref.send({ type: "Increment" });
}
</script>

<div class="flex gap-4 items-center">
  <button
    class="btn btn-primary"
    onclick={handleClick}
  >
    Increase
  </button>

  <div>{actor.state.context.count}</div>
</div>

