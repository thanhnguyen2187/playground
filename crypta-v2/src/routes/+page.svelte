<script lang="ts">
import { globalClient } from "$lib/global";
import {
	type ModalSettings,
	ProgressRadial,
	getModalStore,
} from "@skeletonlabs/skeleton";
import { useMachine } from "@xstate/svelte";
import ModalNote from "../components/ModalNote.svelte";
import NotesList from "../components/NotesList.svelte";
import { notes } from "../data/mock";
import { notesRead, notesUpsert } from "../data/queries-triplit";
import { machine } from "../lib/machine-app";

import { createEmptyNoteDisplay } from "../data/data-transformation";

const modalStore = getModalStore();
const appMachine = useMachine(machine, {});
const { snapshot: appSnapshot, send: appSend } = appMachine;
const modalNote: ModalSettings = {
	type: "component",
	component: {
		ref: ModalNote,
		props: {
			note: $appSnapshot.context.note,
			actionSaveFn: async () => {
				await notesUpsert(globalClient, $appSnapshot.context.note);
				modalStore.close();
        appSend({ type: "Reload" });
        const notes = await notesRead(globalClient, 10);
        appSend({ type: "Loaded", notes });
        console.log(notes)
			},
		},
	},
	response: async () => {
		appSend({ type: "ModalCancel" });
	},
	// type: "confirm",
};

(async () => {
	try {
		const notes = await notesRead(globalClient, 10);
		appSend({ type: "Loaded", notes });
	} catch (e) {
		appSend({ type: "FailedData" });
	}
})();

function modalOpenNote() {
	appSend({ type: "ModalOpenNote", note: createEmptyNoteDisplay() });
	// TODO: try catch in case it doesn't work?
	modalStore.trigger(modalNote);
}
</script>

{JSON.stringify($appSnapshot.value)}

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
