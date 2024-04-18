<script lang="ts">
  import { circleDrawerMachine } from './machine'
  import type { Circle } from './machine'
  import { useMachine } from '@xstate/svelte'
  import { onMount } from 'svelte';

  const { snapshot, send } = useMachine(
    circleDrawerMachine,
    {
      input: {
        renderFn: render,
      },
    }
  )
  let canvas: HTMLCanvasElement

  function render(circles: Circle[], selectedIndex: number) {
    const context = canvas.getContext('2d')
    if (!context) {
      return
    }

    context.clearRect(0, 0, canvas.width, canvas.height)

    circles.forEach((circle, index) => {
      context.beginPath()
      context.arc(circle.x, circle.y, circle.r, 0, 2 * Math.PI)
      context.strokeStyle = 'black'
      context.lineWidth = 1
      context.stroke()

      if (index === selectedIndex) {
        context.fillStyle = 'gray'
        context.fill()
      }
    })
  }

  onMount(() => {
    canvas.width = canvas.offsetWidth
    canvas.height = canvas.offsetHeight
  })

  function handleClick(event: MouseEvent) {
    const rect = canvas.getBoundingClientRect()
    const x = event.clientX - rect.left
    const y = event.clientY - rect.top

    send({
      type: 'circle.select',
      x, y,
    })
  }

  function handleRadiusChange(event: Event) {
    send({
      type: 'circle.mutate',
      r: Number((event.target as HTMLInputElement).value)
    })
  }

  function handleUndo(event: Event) {
    send({type: 'undo'})
  }

  function handleRedo(event: Event) {
    send({type: 'redo'})
  }
</script>

<button
  on:click={handleUndo}
>
  Undo
</button>
<button
  on:click={handleRedo}
>
  Redo
</button> <br/>

<canvas
  bind:this={canvas}
  style="width: 600px; height: 300px; border: 1px solid black;"
  on:mousedown={handleClick}
></canvas>

<br/>

{#if $snapshot.context.selectedIndex !== -1}
  <span>Adjust diameter of circle</span> <br/>
  <input
    min="0" max="100"
    value={$snapshot.context.circles[$snapshot.context.selectedIndex].r}
    type="range"
    on:input={handleRadiusChange}
    on:change={handleRadiusChange}
  /> <br/>
{/if}

<a href="/">Back</a>
