<script lang="ts">
  import { cellsMachine, generateKey } from './machine'
  import { useMachine } from '@xstate/svelte'
  import { createDisplayState } from './display-state';
  import { derived } from 'svelte/store';

  const { snapshot, send } = useMachine(cellsMachine)
  const contextStore = derived(snapshot, snapshot_ => snapshot_.context)
  const displayState = createDisplayState(contextStore)

  function handleCellChange(e: Event, rowIndex: number, columnIndex: number) {
    const value = (e.target as HTMLInputElement).value
    const key = generateKey(rowIndex, columnIndex)
    send({
      type: 'cellInputs.set',
      key, value,
    })
  }
</script>

{JSON.stringify($contextStore)}

<table>
  <thead>
    <tr>
      <th></th>
      {#each $displayState.columnHeaders as columnHeader}
        <th>{columnHeader}</th>
      {/each}
    </tr>
  </thead>
  <tbody>
    {#each $displayState.rowHeaders as rowHeader, rowIndex}
      <tr>
        <th>{rowHeader}</th>
        {#each $displayState.cellValues[rowIndex] as cellValue, columnIndex}
          <td>
            <input
              style="width: 3em"
              value={cellValue}
              on:change={e => handleCellChange(e, rowIndex, columnIndex)}
            />
          </td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>

<a href="/">Back</a>
