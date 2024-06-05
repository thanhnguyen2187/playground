<script lang="ts">
import NotesList from "../components/NotesList.svelte";
import { notes } from "../data/mock";
</script>

<div class="container mt-4 mx-auto flex justify-center items-center">
  <NotesList notes={notes}></NotesList>
</div>

<style lang="postcss"></style>
