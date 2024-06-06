<script lang="ts">
import {
	faClipboard,
	faClone,
	faCopy,
	faEdit,
	faLock,
	faUnlock,
	faKey,
	faTrashCan,
  faChain,
} from "@fortawesome/free-solid-svg-icons";
import { getToastStore } from '@skeletonlabs/skeleton';
import { Fa } from "svelte-fa";
import { useMachine } from "@xstate/svelte";
import { machine } from "$lib/machine-note-item";
import { InputChip } from '@skeletonlabs/skeleton';

export let title = "Unnamed";
let state: "idling" | "locked" | "unlocked" = "idling";

const toastStore = getToastStore();

function sendEventEncrypted() {
  state = "locked";
  toastStore.trigger({
    message: "Note encrypted",
    background: "variant-filled-success",
  })
}

function sendEventDecrypted() {
  state = "unlocked";
}

function sendEventCleared() {
  state = "idling";
}
</script>

<div class="bg-surface-500 p-2 border rounded flex justify-between gap-2">
  <div class="w-60 flex items-center">
    <span class="truncate">{title}</span>
  </div>
  <div class="w-40 truncate">
      <span class="chip variant-ghost-secondary">tag-1</span>
      <span class="chip variant-ghost-secondary">tag-2</span>
      <span class="chip variant-ghost-secondary">tag-3</span>
    </div>
  <div class="w-40 flex items-center">
    {new Date().toLocaleString()}
  </div>
  <div class="flex items-center gap-2">
    {#if state === "idling"}
      <button>
        <Fa icon={faEdit}></Fa>
      </button>
      <button>
        <Fa icon={faCopy}></Fa>
      </button>
      <button on:click={sendEventEncrypted}>
        <Fa icon={faLock}></Fa>
      </button>
    {:else if state === "locked"}
      <button on:click={sendEventDecrypted}>
        <Fa icon={faUnlock}></Fa>
      </button>
      <button on:click={sendEventCleared}>
        <Fa icon={faKey}></Fa>
      </button>
    {:else if state === "unlocked"}
      <button>
        <Fa icon={faEdit}></Fa>
      </button>
      <button>
      <Fa icon={faCopy}></Fa>
      </button>
      <button on:click={sendEventCleared}>
        <Fa icon={faKey}></Fa>
      </button>
    {/if}
    <button>
      <Fa icon={faTrashCan}></Fa>
    </button>
  </div>
</div>
