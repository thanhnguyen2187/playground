<script lang="ts">
  import ItemRow from "./ItemRow.svelte"
  import { createNewItem } from "./data"
  import type { Item } from "./data"
  import { onDestroy } from 'svelte';

  export let items: Item[] = []
  export let saveItemsFn: (items: Item[]) => void
  let newItem = createNewItem()

  function addNewItem() {
    items = [...items, newItem]
    newItem = createNewItem()
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
        <button>+</button>
      </td>
    </tr>

    </tbody>
  </table>
</div>
