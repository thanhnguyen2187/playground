<script lang="ts">
  import ItemRow from "./ItemRow.svelte"
  import { createNewItem } from "./data"
  import type { Item } from "./data"
  import { onDestroy } from 'svelte';

  export let items: Item[] = []
  export let saveItemsFn: (items: Item[]) => void

  function addNewItem() {
    items = [...items, createNewItem()]
  }

  function removeItem(index: number) {
    items = [
      ...items.slice(0, index),
      ...items.slice(index + 1),
    ]
  }

  onDestroy(() => {
    saveItemsFn(items)
  })
</script>

<div class="w-modal">
  <table class="table table-hover">
    <thead>
    <tr>
      <th>Text</th>
      <th>Colors</th>
      <th>Preview</th>
      <th></th>
    </tr>
    </thead>
    <tbody>
    {#each items as item, index (item.key)}
      <ItemRow
        newRow={false}
        item={item}
        addItemFn={() => {}}
        removeItemFn={() => removeItem(index)}
      />
    {/each}
    <tr>
      <td></td>
      <td></td>
      <td></td>
      <td>
        <button
          class="btn btn-icon p-2 variant-ghost"
          on:click={addNewItem}
        >
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="48" height="48" color="#000000" fill="none">
            <path d="M12 4V20" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            <path d="M4 12H20" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </button>
      </td>
    </tr>

    </tbody>
  </table>
</div>
