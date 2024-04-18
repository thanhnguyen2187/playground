<script lang="ts">
  let count = 0

  function increase() {
      count += 1
  }
</script>

<div>
  <input type="number" bind:value={count}/>
  <button on:click={increase}>Increase</button>
</div>
