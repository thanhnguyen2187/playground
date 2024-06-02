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
import { Fa } from "svelte-fa";
import { useMachine } from "@xstate/svelte";
import { machine } from "$lib/machine-note-item";

export let title = "Unnamed";

const { snapshot, send } = useMachine(machine);

function sendEventEncrypted() {
	send({ type: "Encrypted" });
}

function sendEventDecrypted() {
	send({ type: "Decrypted" });
}

function sendEventCleared() {
  send({ type: "Cleared" });
}
</script>

<div class="bg-surface-500 p-2 border rounded flex justify-between gap-2 w-80">
  <div>
    {title}
  </div>
  <div class="flex items-center gap-2">
    {#if $snapshot.matches('Idling')}
      <Fa icon={faEdit}></Fa>
      <Fa icon={faCopy}></Fa>
      <button on:click={sendEventEncrypted}>
        <Fa icon={faLock}></Fa>
      </button>
    {:else if $snapshot.matches('Locked')}
      <button on:click={sendEventDecrypted}>
        <Fa icon={faUnlock}></Fa>
      </button>
      <button on:click={sendEventCleared}>
        <Fa icon={faKey}></Fa>
      </button>
    {:else if $snapshot.matches('Unlocked')}
      <Fa icon={faEdit}></Fa>
      <Fa icon={faCopy}></Fa>
      <button on:click={sendEventCleared}>
        <Fa icon={faKey}></Fa>
      </button>
    {/if}
    <Fa icon={faTrashCan}></Fa>
  </div>
</div>
