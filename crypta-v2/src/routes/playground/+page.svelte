<script lang="ts">
import NoteItemList from "../../components/NoteItemList.svelte";
import { TriplitClient } from "@triplit/client";

const client = new TriplitClient();

let result = "loading...";

(async () => {
	const insertResult = await client.insert("notes", { title: "Hello world" });
	await client.insert("notes", { title: "Goodbye world" });
	const query = client.query("notes").select(["id", "title"]).build();
	result = await client.fetch(query, { policy: "local-only" });
})();
</script>

<div class="container h-full mx-auto flex justify-center items-center">
  <NoteItemList/>
  {JSON.stringify(result)}
</div>
