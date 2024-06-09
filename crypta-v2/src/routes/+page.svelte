<script lang="ts">
import { globalClient } from "$lib/global";
import { faAdd, faGear } from "@fortawesome/free-solid-svg-icons";
import {
	type ModalSettings,
	ProgressRadial,
	getModalStore,
	getToastStore,
} from "@skeletonlabs/skeleton";
import { useMachine } from "@xstate/svelte";
import { Fa } from "svelte-fa";
import ModalNote from "../components/ModalNote.svelte";
import ModalEncryption from "../components/ModalEncryption.svelte";
import NotesList from "../components/NotesList.svelte";
import {
	createEmptyNoteDisplay,
	encryptNote,
} from "../data/data-transformation";
import { notes } from "../data/mock";
import { notesDelete, notesRead, notesUpsert } from "../data/queries-triplit";
import { machine } from "../lib/machine-app";
import type { NoteDisplay } from "../data/schema-triplit";

const modalStore = getModalStore();
const toastStore = getToastStore();
const appMachine = useMachine(machine, {});
const { snapshot: appSnapshot, send: appSend } = appMachine;

async function itemsLoad() {
	try {
		const notes = await notesRead(globalClient, 10);
		appSend({ type: "Loaded", notes });
	} catch (e) {
		appSend({ type: "FailedData" });
		console.error(e);
	}
}

function fnModalClose() {
	modalStore.close();
	appSend({ type: "ModalCancel" });
}

function fnModalOpenNoteCreate() {
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
				fnCancel: fnModalClose,
				fnSubmit: async () => {
					await notesUpsert(globalClient, note);
					appSend({ type: "Reload" });
					modalStore.close();
					await itemsLoad();
				},
			},
		},
		response: () => appSend({ type: "ModalCancel" }),
	});
}

function fnModalOpenNoteUpdate(note: NoteDisplay) {
	appSend({ type: "ModalOpenNote", note });
	modalStore.trigger({
		type: "component",
		component: {
			ref: ModalNote,
			props: {
				note,
				fnCancel: fnModalClose,
				fnSubmit: async () => {
					await notesUpsert(globalClient, note);
					appSend({ type: "Reload" });
					modalStore.close();
					await itemsLoad();
				},
			},
		},
		response: () => appSend({ type: "ModalCancel" }),
	});
}

function fnModalEncryption(note: NoteDisplay) {
	appSend({ type: "ModalOpenEncryption", note });
	modalStore.trigger({
		type: "component",
		component: {
			ref: ModalEncryption,
			props: {
				note,
				fnCancel: fnModalClose,
				fnSubmit: async (password: string) => {
					const encryptedNote = await encryptNote(note, password);
					await notesUpsert(globalClient, encryptedNote);
					toastStore.trigger({
						message: "Encrypted successfully!",
						background: "variant-ghost-success",
						timeout: 2000,
					});
					appSend({ type: "Reload" });
					modalStore.close();
					await itemsLoad();
				},
			},
		},
		response: () => appSend({ type: "ModalCancel" }),
	});
}

function fnModalConfirmDeletion(noteId: string) {
  appSend({ type: "ModalConfirmDeletion" });
  modalStore.trigger({
    type: "confirm",
    body: "Are you sure? Once the note is deleted, it can be hard to be retrieved!",
    modalClasses: "!w-modal-slim",
    response: async (result: boolean) => {
      appSend({ type: "ModalCancel" })
      if (result) {
        await notesDelete(globalClient, noteId);
        appSend({ type: "Reload" });
        await itemsLoad();
      }
      modalStore.close();
    },
  });
}

itemsLoad();
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
    on:click={fnModalOpenNoteCreate}
  >
    <Fa icon={faAdd} size="lg"/>
  </button>
{/if}

<div class="container mt-6 mx-auto flex justify-center items-center">
  {#if $appSnapshot.matches("Functioning.Loading")}
  <ProgressRadial value={undefined} />
  {:else if $appSnapshot.matches("Functioning.Idling.Items.Blank")}
    <p>
      There is nothing here for now.<br/>
      You might want to <button class="underline" on:click={fnModalOpenNoteCreate}>create one</button>?
    </p>
  {:else if $appSnapshot.matches("Functioning.Idling.Items.Filled")}
    <NotesList
      notes={$appSnapshot.context.notes}
      fnUpdate={fnModalOpenNoteUpdate}
      fnEncrypt={fnModalEncryption}
      fnDelete={fnModalConfirmDeletion}
    />
  {:else}
    Error
  {/if}
</div>

<style lang="postcss"></style>
