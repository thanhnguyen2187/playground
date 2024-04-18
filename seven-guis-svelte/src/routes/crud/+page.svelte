<script lang="ts">
  type Record = {
    name: string,
    surname: string,
  }
  let records: Record[] = [
    {name: 'Emil', surname: 'Hans'},
    {name: 'Mustermann', surname: 'Max'},
    {name: 'Tisch', surname: 'Roman'},
  ]
  let selectedRecordIndex = -1
  let selectedRecordData: Record = {
    name: '',
    surname: '',
  }
  let filterPrefix = ''

  function readRecord(index: number): Record {
    return records[index] ?? {name: '', surname: ''}
  }

  function setRecordField(fieldName: string, value: string) {
    selectedRecordData[fieldName] = value
  }

  function createRecord() {
    records = [
      ...records,
      {...selectedRecordData}
    ]
  }

  function deleteRecord() {
    if (selectedRecordIndex !== -1) {
      // A spread syntax version would look a lot nicer, but Svelte compiler somehow had a bug at the time that this was
      // implemented time. Also see: https://github.com/sveltejs/svelte/issues/7985
      const recordsCopy = records.slice()
      recordsCopy.splice(selectedRecordIndex, 1)
      records = recordsCopy
    }
  }

  function updateRecord() {
    if (selectedRecordIndex !== -1) {
      records[selectedRecordIndex] = selectedRecordData
    }
  }
</script>

<div>
  <label>
    Filter prefix:
    <input type="text" bind:value={filterPrefix}/>
  </label> <br/>

  <select
    on:change={
      e => {
        selectedRecordIndex = e.target.value
        selectedRecordData = {...records[selectedRecordIndex]}
      }
    }
    size="4"
  >
    {#each records as record, index}
      {#if record.name.toLowerCase().startsWith(filterPrefix.toLowerCase())}
        <option value={index}>{record.name}, {record.surname}</option>
      {/if}
    {/each}
  </select> <br/>

  <label>
    Name:
    <input
      type="text"
      value={readRecord(selectedRecordIndex).name}
      on:input={e => setRecordField('name', e.target.value)}
    />
  </label> <br/>
  <label>
    Surname:
    <input
      type="text"
      value={readRecord(selectedRecordIndex).surname}
      on:input={e => setRecordField('surname', e.target.value)}
    />
  </label> <br/>

  <button on:click={e => createRecord()}>Create</button>
  <button on:click={e => updateRecord()}>Update</button>
  <button on:click={e => deleteRecord()}>Delete</button>
</div>
