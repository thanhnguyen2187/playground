<script lang="ts">
  import { onMount } from 'svelte'

  type Circle = {
    state: 'default' | 'highlighted'
    radius: number
    x: number
    y: number
  }

  const defaultRadius = 15

  let circlesHistory: Circle[][] = [[]]
  let circlesHistoryIndex: number = 0

  let circles: Circle[] = []
  let highlightedCircleIndex: number = -1
  let highlightedCircle: Circle | undefined
  $: highlightedCircle = circles.filter(
    circle => circle.state === 'highlighted'
  )[0]

  let canvas: HTMLCanvasElement
  let rect: DOMRect
  let context: CanvasRenderingContext2D
  let x: number
  let y: number

  onMount(() => {
    context = canvas.getContext("2d")
    rect = canvas.getBoundingClientRect()
  })

  function square(x: number): number {
    return x * x
  }

  function clickedWithinCircles(x: number, y: number, circles: Circle[]): number {
    return circles.findIndex(
      circle => {
        // TODO: Check why must the radius be multiplied by 16 in the formula
        //       The reason is related to `context.arc`, where `circle.radius` is used
        return square(x - circle.x) + square(y - circle.y) < circle.radius * 16
      }
    )
  }

  function invertState(circle: Circle): Circle {
    switch (circle.state) {
      case 'default':
        return {
          ...circle,
          state: 'highlighted',
        }
      case 'highlighted':
        return {
          ...circle,
          state: 'default',
        }
      default:
        return circle
    }
  }

  function dehighlightCircles(circles: Circle[]): Circle[] {
    return circles.map(
      circle => ({
        ...circle,
        state: 'default',
      })
    )
  }

  function handleClick(event: MouseEvent) {
    x = event.clientX - rect.left
    y = event.clientY - rect.top

    highlightedCircleIndex = clickedWithinCircles(x, y, circles)
    if (highlightedCircleIndex === -1) {
      circles = [
        ...dehighlightCircles(circles),
        {
          state: 'highlighted',
          radius: defaultRadius,
          x, y,
        }
      ]
      highlightedCircleIndex = circles.length - 1
      circlesHistoryIndex += 1
      circlesHistory = [
        ...circlesHistory.slice(0, circlesHistoryIndex),
        circles,
      ]
    } else {
      if (circles[highlightedCircleIndex].state === 'default') {
        circles = dehighlightCircles(circles)
      }
      circles[highlightedCircleIndex] = invertState(circles[highlightedCircleIndex])
    }

    render()
  }

  function handleRadiusChange(event: InputEvent) {
    const target: HTMLInputElement = event.target as HTMLInputElement
    circles[highlightedCircleIndex] = {
      ...circles[highlightedCircleIndex],
      radius: Number.parseInt(target.value)
    }

    // TODO: improve by throttling history creating
    circlesHistoryIndex += 1
    circlesHistory = [
      ...circlesHistory.slice(0, circlesHistoryIndex),
      circles,
    ]

    render()
  }

  function undo() {
    const nextCirclesHistoryIndex = circlesHistoryIndex - 1
    if (nextCirclesHistoryIndex >= 0) {
      circlesHistoryIndex = nextCirclesHistoryIndex
      circles = circlesHistory[circlesHistoryIndex]
    }
    circles = dehighlightCircles(circles)
    render()
  }

  function redo() {
    const nextCirclesHistoryIndex = circlesHistoryIndex + 1
    if (nextCirclesHistoryIndex < circlesHistory.length) {
      circlesHistoryIndex = nextCirclesHistoryIndex
      circles = circlesHistory[circlesHistoryIndex]
    }
    circles = dehighlightCircles(circles)
    render()
  }

  function render() {
    context.clearRect(0, 0, canvas.width, canvas.height)

    circles.forEach(
      circle => {
        context.beginPath()
        context.arc(
          circle.x,
          circle.y,
          circle.radius,
          0, 2 * Math.PI,
        )
        switch (circle.state) {
          case 'default':
            context.stroke()
            break
          case 'highlighted':
            context.stroke()
            context.fillStyle = 'gray'
            context.fill()
            break
        }
        context.closePath()
      }
    )
  }

</script>

<div>
  <button
    on:click={e => undo()}
  >
    Undo
  </button>
  <button
    on:click={e => redo()}
  >
    Redo
  </button> <br />
  <canvas
    bind:this={canvas}
    style="border: 1px solid black;"
    width="500"
    height="300"
    on:click={handleClick}
  ></canvas> <br/>

  {#if !!highlightedCircle}
    Adjust diameter of circle at ({highlightedCircle.x} {highlightedCircle.y}) <br/>
    <input
      min="10"
      max="100"
      type="range"
      value={highlightedCircle.radius}
      on:input={handleRadiusChange}
    />
  {/if}
</div>
