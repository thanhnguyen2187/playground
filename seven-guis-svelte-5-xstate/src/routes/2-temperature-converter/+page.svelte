<script lang="ts">
import { machine } from "./state";
import { createActor } from "xstate";
import { wrap } from "$lib/xstate-wrap.svelte";

const actor = wrap(createActor(machine));

function handleInputC(event: Event) {
  const target = event.currentTarget as HTMLInputElement;
  actor.ref.send({
    type: "InputC",
    value: Number(target.value),
  });
}

function handleInputF(event: Event) {
  const target = event.currentTarget as HTMLInputElement;
  actor.ref.send({
    type: "InputF",
    value: Number(target.value),
  });
}
</script>

<div class="flex gap-2 items-center">
  <div>
    <input
      type="number"
      class="input input-bordered w-32"
      oninput={handleInputC}
      value={actor.state.context.currentC.value}
    />
  </div>
  <div>Celsius</div>
  <div>
    =
  </div>
  <div>
    <input
      type="number"
      class="input input-bordered w-32"
      oninput={handleInputF}
      value={actor.state.context.currentF.value}
    />
  </div>
  <div>Fahrenheit</div>
</div>
