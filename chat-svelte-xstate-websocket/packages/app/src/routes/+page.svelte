<script lang="ts">
import { machine } from "./state";
import { createActor } from "xstate";
import { wrap } from "$lib/xstate-wrapper.svelte";

const actor = wrap(createActor(machine));
</script>

<div class="h-screen bg-base-200 p-4">
  <div class="max-w-2xl mx-auto flex flex-col h-full">
    <!-- Messages Area -->
    <div class="flex-1 bg-base-100 rounded-lg p-4 mb-4 overflow-auto">
      {#each actor.state.context.messages as message}
        <div class="chat chat-start">
          <div class="chat-bubble">{message}</div>
        </div>
      {/each}
    </div>

    <!-- Input Area -->
    <div class="flex gap-2">
      <input type="text" placeholder="Type a message..." class="input input-bordered flex-1" />
      <button class="btn btn-primary">Send</button>
    </div>
  </div>
</div>
