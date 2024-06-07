<script lang="ts">
import NoteItemList from "./NoteItemList.svelte";
import type { NoteDisplay } from "../data/schema-triplit";

export let notes: NoteDisplay[] = [];
export let fnUpdate: (note: NoteDisplay) => void;
export let fnEncrypt: (note: NoteDisplay) => void;
</script>

<div class="flex flex-col gap-2">
  {#each notes as note(note.id)}
    <NoteItemList {note} {fnUpdate} {fnEncrypt} />
  {/each}
</div>
