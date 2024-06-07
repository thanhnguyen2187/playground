<script lang="ts">
import { faClose } from "@fortawesome/free-solid-svg-icons";
import { InputChip } from "@skeletonlabs/skeleton";
import { Fa } from "svelte-fa";
import type { NoteDisplay } from "../data/schema-triplit";

export let note: NoteDisplay;
export let actionSaveFn: () => void;
</script>

<div class="card w-modal relative">
  <button
    class="badge-icon variant-ghost-secondary absolute top-3 right-3"
  >
    <Fa icon={faClose} />
  </button>
  <section class="p-4 flex flex-col gap-2">
    <label class="label">
      <span>Title</span>
      <input
        class="input"
        spellcheck="false"
        bind:value={note.title}
      />
    </label>
    <label class="label">
      <span>Text</span>
      <textarea
        class="textarea"
        rows="8"
        spellcheck="false"
        bind:value={note.text}
      ></textarea>
    </label>
    <label>
      <span>Tags</span>
      <InputChip name="tags" bind:value={note.tags}/>
    </label>
    <label class="label">
      <span>ID</span>
      <input
        class="input"
        value={note.id}
        disabled
      />
    </label>
    <label class="label">
      <span>Date created</span>
      <input
        value={note.createdAt.toLocaleString()}
        class="input"
        disabled
      />
    </label>
    <label class="label">
      <span>Last updated</span>
      <input
        value={note.updatedAt.toLocaleString()}
        class="input"
        disabled
      />
    </label>
  </section>
  <footer class="card-footer flex flex-row-reverse">
    <button
      class="btn variant-filled"
      on:click={actionSaveFn}
    >
      Save
    </button>
  </footer>
</div>
