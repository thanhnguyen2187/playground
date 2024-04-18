<script lang="ts">
  import { crudMachine } from './machine'
  import { useMachine } from '@xstate/svelte'
  import type { Record } from './machine'

  const { snapshot, send } = useMachine(crudMachine)

  function handleSelectRecord(index: number) {
    send({
      type: 'record.select',
      index,
    })
  }

  function handleMutateRecord(e: Event, key: keyof Record) {
    const value = (e.target as HTMLInputElement).value
    send({
      type: 'record.mutate',
      key,
      value,
    })
  }

  function handleUpdateRecord() {
    send({
      type: 'record.update',
    })
  }

  function handleAddRecord() {
    send({
      type: 'record.add',
    })
  }

  function handleSetFilter(e: Event) {
    send({
      type: 'filter.set',
      value: (e.target as HTMLInputElement).value,
    })
  }

  function handleDelete() {
    send({
      type: 'record.delete',
    })
  }
</script>

<span>Filter prefix: </span>
<input
  value={$snapshot.context.filter}
  on:keyup={handleSetFilter}
/> <br/>

<div style="display: flex">
  <select
    size="4"
    style="width: 10em"
  >
    {#each $snapshot.context.displayRecords as record (record.index)}
      <option
        value={record.index}
        on:click={() => handleSelectRecord(record.index)}
        selected={record.index === $snapshot.context.selectedIndex}
      >
        {record.name}, {record.surname}
      </option>
    {/each}
  </select>
  <div>
    <span>Name: </span>
    <input
      value={$snapshot.context.selectedRecord.name}
      on:change={(e) => handleMutateRecord(e, 'name')}
    /> <br/>
    <span>Surname: </span>
    <input
      value={$snapshot.context.selectedRecord.surname}
      on:change={(e) => handleMutateRecord(e, 'surname')}
    />
  </div>
</div>

<button on:click={handleAddRecord}>Create</button>
<button on:click={handleUpdateRecord}>Update</button>
<button on:click={handleDelete}>Delete</button>
<br/>

<a href="/">Back</a>
