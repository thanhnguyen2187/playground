<script lang="ts">
let elapsedTime = 0
let duration = 30000

setInterval(
  () => {
    if (elapsedTime < duration) {
      elapsedTime += 100
    }
  },
  100,
)

</script>

<div>
  <span>Elapsed Time: </span>
  <progress value={elapsedTime} max={duration}></progress> <br/>
  {elapsedTime / 1000}s / {duration / 1000}s<br/>
  <input
    min="0"
    max="60"
    type="range"
    on:change={e => duration = e.target.value * 1000}
  /> <br/>
  <button
    on:click={() => elapsedTime = 0}
  >
    Reset
  </button>
</div>
