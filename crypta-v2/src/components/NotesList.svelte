<script lang="ts">
import NoteItemList from "./NoteItemList.svelte";
import type { NoteDisplay } from "../data/schema-triplit";

export let notes: NoteDisplay[] = [];
</script>

<div class="flex flex-col gap-2">
  {#each notes as note(note.id)}
    <NoteItemList title={note.title} />
  {/each}
</div>
