<script lang="ts">
import { globalClient } from "$lib/global";
import { faAdd, faGear } from "@fortawesome/free-solid-svg-icons";
import {
	type ModalSettings,
	ProgressRadial,
	getModalStore,
} from "@skeletonlabs/skeleton";
import { useMachine } from "@xstate/svelte";
import { Fa } from "svelte-fa";
import ModalNote from "../components/ModalNote.svelte";
import NotesList from "../components/NotesList.svelte";
import { createEmptyNoteDisplay } from "../data/data-transformation";
import { notes } from "../data/mock";
import { notesRead, notesUpsert } from "../data/queries-triplit";
import { machine } from "../lib/machine-app";

const modalStore = getModalStore();
const appMachine = useMachine(machine, {});
const { snapshot: appSnapshot, send: appSend } = appMachine;

(async () => {
	try {
		const notes = await notesRead(globalClient, 10);
		appSend({ type: "Loaded", notes });
	} catch (e) {
		appSend({ type: "FailedData" });
	}
})();

function modalOpenNote() {
  // NOTE: this is kind of a... magic way to do it, as we use Svelte's own
  //       reactivity system to mutate `note` within the opened component.
	const note = createEmptyNoteDisplay();
	appSend({ type: "ModalOpenNote", note });
	// TODO: try catch in case it doesn't work?
	modalStore.trigger({
		type: "component",
		component: {
			ref: ModalNote,
			props: {
				note,
				actionSaveFn: async () => {
					await notesUpsert(globalClient, note);
					modalStore.close();
					appSend({ type: "Reload" });
					const notes = await notesRead(globalClient, 10);
					appSend({ type: "Loaded", notes });
				},
			},
		},
		response: async () => {
			appSend({ type: "ModalCancel" });
		},
	});
}
</script>

{JSON.stringify($appSnapshot.value)}

{#if $appSnapshot.matches("Functioning.Idling")}
  <button
    class="btn-icon variant-filled-secondary absolute bottom-6 left-6"
  >
    <Fa icon={faGear} size="lg"/>
  </button>
  <button
    class="btn-icon variant-filled-secondary absolute bottom-6 right-6"
    on:click={modalOpenNote}
  >
    <Fa icon={faAdd} size="lg"/>
  </button>
{/if}

<div class="container mt-6 mx-auto flex justify-center items-center">
  {#if $appSnapshot.matches("Functioning.Loading")}
  <ProgressRadial value={undefined} />
  {:else if $appSnapshot.matches("Functioning.Idling.Items.Blank")}
    <span>
      There is nothing here for now.<br/>
      You might want to <button class="underline" on:click={modalOpenNote}>create one</button>?
    </span>
  {:else if $appSnapshot.matches("Functioning.Idling.Items.Filled")}
    <NotesList notes={$appSnapshot.context.notes} />
  {:else}
    Error
  {/if}
</div>

<style lang="postcss"></style>
