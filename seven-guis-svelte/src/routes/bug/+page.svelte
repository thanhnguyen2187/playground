<script lang="ts">
  let numbers = [1, 2, 3, 4, 5]
  let selectedIndex = -1
  let currentNumber = 0

function deleteSelected() {
  if (selectedIndex !== -1) {
    // this doesn't work
    numbers = [
      ...numbers.slice(0, selectedIndex),
      ...numbers.slice(Number.parseInt(selectedIndex.toString()) + 1),
    ]

    // this does
    // const numbersCopy = numbers.slice()
    // numbersCopy.splice(selectedIndex, 1)
    // numbers = numbersCopy
  }
}
</script>

<select
  size="8"
  on:change={e => { selectedIndex = e.target.value; currentNumber = numbers[selectedIndex] }}
>
  {#each numbers as number, index}
    <option value={index} style="width: 100px">
      {number}
    </option>
  {/each}
</select> <br/>

<input
  type="number"
  value={currentNumber}
/> <br/>

<button>Create</button>
<button>Update</button>
<button on:click={e => deleteSelected()}>Delete</button>

<hr/>

Current state: <br/>

- Selected index: {selectedIndex} <br/>
- Elements: {JSON.stringify(numbers)} <br/>
