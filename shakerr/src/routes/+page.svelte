<script lang="ts">
  import { machine } from '$lib/machine'
  import { useMachine } from '@xstate/svelte'
  import { getModalStore } from '@skeletonlabs/skeleton'
  import SettingsModal from '$lib/SettingsModal.svelte'
  import { derived, writable } from 'svelte/store'
  import type { Item } from '$lib/data'
  import { addShakeTracking } from '$lib/shake'

  const {snapshot: snapshotStore, send} = useMachine(machine)
  const modalStore = getModalStore()

  addShakeTracking(() => send({type: 'Shaked'}))

  $: {
    if ($snapshotStore.value === 'Desktop Warning') {
      modalStore.trigger({
        type: 'alert',
        title: 'Warning',
        body: 'You are using Shakerr on a desktop device. Please use a mobile device for best experience!',
        buttonTextCancel: 'OK',
        response: () => send({type: 'Accept'})
      })
    } else if (typeof $snapshotStore.value === 'object' && 'Settings' in $snapshotStore.value) {
      modalStore.trigger({
        type: 'component',
        component: {
          ref: SettingsModal,
          props: {
            items: $snapshotStore.context.items,
            saveItemsFn: (items: Item[]) => {
              send({type: 'Exit', items})
            }
          }
        },
      })
    }
  }

  function shake() {
    send({type: 'Shaked'})
  }
  function reset() {
    send({type: 'Reset'})
  }

  const tick = writable(0, () => {
    let interval = setInterval(() => {
      tick.update(value => value + 1)
    }, 500)

    return () => {
      clearInterval(interval)
    }
  })

  const noData = derived(snapshotStore, snapshot => typeof snapshot.value === 'object' && snapshot.value['Functioning'] === 'No Data')
  const noResult = derived(snapshotStore, snapshot => typeof snapshot.value === 'object' && snapshot.value['Functioning'] === 'No Result')
  const shaking = derived(snapshotStore, snapshot => typeof snapshot.value === 'object' && snapshot.value['Functioning'] === 'Shaking')
  const showResult = derived(snapshotStore, snapshot => typeof snapshot.value === 'object' && snapshot.value['Functioning'] === 'Show Result')
  const result = derived(snapshotStore, snapshot => snapshot.context.items[snapshot.context.pickedIndex])
  const backgroundColors = derived(snapshotStore, snapshot => snapshot.context.items.map(item => item.backgroundColor))
  const colorStyle = derived(result, (item) => {
    if (!item) {
      return ``
    }

    return `background-color: ${item.backgroundColor}; color: ${item.textColor}`
  })
  const textColor = derived(result, (item) => {
    if (!item) {
      return `#000000`
    }

    return item.textColor
  })
</script>

<!--<div class="absolute">-->
<!--  {JSON.stringify($snapshotStore.value)} <br/>-->
<!--  {JSON.stringify($backgroundColors)}-->
<!--  {JSON.stringify($snapshotStore.context)}-->
<!--</div>-->

<div class="absolute bottom-2 left-3 z-10">
  <button
    class="btn btn-icon p-2 variant-ghost"
    on:click={() => send({type: 'Open Settings'})}
  >
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="48" height="48" color={$textColor} fill="none">
      <path
        d="M15.5 12C15.5 13.933 13.933 15.5 12 15.5C10.067 15.5 8.5 13.933 8.5 12C8.5 10.067 10.067 8.5 12 8.5C13.933 8.5 15.5 10.067 15.5 12Z"
        stroke="currentColor" stroke-width="1.5"/>
      <path
        d="M21.011 14.0965C21.5329 13.9558 21.7939 13.8854 21.8969 13.7508C22 13.6163 22 13.3998 22 12.9669V11.0332C22 10.6003 22 10.3838 21.8969 10.2493C21.7938 10.1147 21.5329 10.0443 21.011 9.90358C19.0606 9.37759 17.8399 7.33851 18.3433 5.40087C18.4817 4.86799 18.5509 4.60156 18.4848 4.44529C18.4187 4.28902 18.2291 4.18134 17.8497 3.96596L16.125 2.98673C15.7528 2.77539 15.5667 2.66972 15.3997 2.69222C15.2326 2.71472 15.0442 2.90273 14.6672 3.27873C13.208 4.73448 10.7936 4.73442 9.33434 3.27864C8.95743 2.90263 8.76898 2.71463 8.60193 2.69212C8.43489 2.66962 8.24877 2.77529 7.87653 2.98663L6.15184 3.96587C5.77253 4.18123 5.58287 4.28891 5.51678 4.44515C5.45068 4.6014 5.51987 4.86787 5.65825 5.4008C6.16137 7.3385 4.93972 9.37763 2.98902 9.9036C2.46712 10.0443 2.20617 10.1147 2.10308 10.2492C2 10.3838 2 10.6003 2 11.0332V12.9669C2 13.3998 2 13.6163 2.10308 13.7508C2.20615 13.8854 2.46711 13.9558 2.98902 14.0965C4.9394 14.6225 6.16008 16.6616 5.65672 18.5992C5.51829 19.1321 5.44907 19.3985 5.51516 19.5548C5.58126 19.7111 5.77092 19.8188 6.15025 20.0341L7.87495 21.0134C8.24721 21.2247 8.43334 21.3304 8.6004 21.3079C8.76746 21.2854 8.95588 21.0973 9.33271 20.7213C10.7927 19.2644 13.2088 19.2643 14.6689 20.7212C15.0457 21.0973 15.2341 21.2853 15.4012 21.3078C15.5682 21.3303 15.7544 21.2246 16.1266 21.0133L17.8513 20.034C18.2307 19.8187 18.4204 19.711 18.4864 19.5547C18.5525 19.3984 18.4833 19.132 18.3448 18.5991C17.8412 16.6616 19.0609 14.6226 21.011 14.0965Z"
        stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
    </svg>
  </button>
</div>

<div class="h-screen w-screen flex flex-col items-center justify-center transition-colors" style={$colorStyle}>
  {#if $noData}
    <p class="">
      Shakerr requires at least two items to work. <br/>
      Please specify them in the settings.
    </p>
  {:else if $noResult}
    <button
      class="btn btn-lg"
      on:click={shake}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="48" height="48" color={$textColor} fill="none">
        <path d="M5.5 9C5.5 5.70017 5.5 4.05025 6.4519 3.02513C7.40381 2 8.93587 2 12 2C15.0641 2 16.5962 2 17.5481 3.02513C18.5 4.05025 18.5 5.70017 18.5 9V15C18.5 18.2998 18.5 19.9497 17.5481 20.9749C16.5962 22 15.0641 22 12 22C8.93587 22 7.40381 22 6.4519 20.9749C5.5 19.9497 5.5 18.2998 5.5 15V9Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        <path d="M12 19H12.009" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M11 5H13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M22 8.5C22 8.5 21 8.846 21 9.8125C21 10.779 22 11.0335 22 12C22 12.9665 21 13.221 21 14.1875C21 15.154 22 15.5 22 15.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M2 8.5C2 8.5 3 8.846 3 9.8125C3 10.779 2 11.0335 2 12C2 12.9665 3 13.221 3 14.1875C3 15.154 2 15.5 2 15.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <span>Shake it!</span>
    </button>
  {:else if $shaking}
    <button
      class="btn btn-lg"
      on:click={shake}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="48" height="48" color={$textColor} fill="none">
        <path d="M5.5 9C5.5 5.70017 5.5 4.05025 6.4519 3.02513C7.40381 2 8.93587 2 12 2C15.0641 2 16.5962 2 17.5481 3.02513C18.5 4.05025 18.5 5.70017 18.5 9V15C18.5 18.2998 18.5 19.9497 17.5481 20.9749C16.5962 22 15.0641 22 12 22C8.93587 22 7.40381 22 6.4519 20.9749C5.5 19.9497 5.5 18.2998 5.5 15V9Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        <path d="M12 19H12.009" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M11 5H13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M22 8.5C22 8.5 21 8.846 21 9.8125C21 10.779 22 11.0335 22 12C22 12.9665 21 13.221 21 14.1875C21 15.154 22 15.5 22 15.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M2 8.5C2 8.5 3 8.846 3 9.8125C3 10.779 2 11.0335 2 12C2 12.9665 3 13.221 3 14.1875C3 15.154 2 15.5 2 15.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <span>Shaking it, but feel free to prolong the process!</span>
    </button>
  {:else if $showResult}
    <button
      class="btn btn-lg"
      on:click={shake}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="48" height="48" color={$textColor} fill="none">
        <path d="M5.5 9C5.5 5.70017 5.5 4.05025 6.4519 3.02513C7.40381 2 8.93587 2 12 2C15.0641 2 16.5962 2 17.5481 3.02513C18.5 4.05025 18.5 5.70017 18.5 9V15C18.5 18.2998 18.5 19.9497 17.5481 20.9749C16.5962 22 15.0641 22 12 22C8.93587 22 7.40381 22 6.4519 20.9749C5.5 19.9497 5.5 18.2998 5.5 15V9Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        <path d="M12 19H12.009" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M11 5H13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M22 8.5C22 8.5 21 8.846 21 9.8125C21 10.779 22 11.0335 22 12C22 12.9665 21 13.221 21 14.1875C21 15.154 22 15.5 22 15.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M2 8.5C2 8.5 3 8.846 3 9.8125C3 10.779 2 11.0335 2 12C2 12.9665 3 13.221 3 14.1875C3 15.154 2 15.5 2 15.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <span>Shake again?</span>
    </button>
    {#if $result.value}
      <div class="text-lg">
        The result you got is: "{$result.value}"
      </div>
    {/if}
    <button
      class="btn btn-lg"
      on:click={reset}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="48" height="48" color="{$textColor}" fill="none">
        <path d="M13.5 6V2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M12 7.5C12 7.03406 12 6.80109 12.0761 6.61732C12.1776 6.37229 12.3723 6.17761 12.6173 6.07612C12.8011 6 13.0341 6 13.5 6C13.9659 6 14.1989 6 14.3827 6.07612C14.6277 6.17761 14.8224 6.37229 14.9239 6.61732C15 6.80109 15 7.03406 15 7.5V9.5C15 9.96594 15 10.1989 14.9239 10.3827C14.8224 10.6277 14.6277 10.8224 14.3827 10.9239C14.1989 11 13.9659 11 13.5 11C13.0341 11 12.8011 11 12.6173 10.9239C12.3723 10.8224 12.1776 10.6277 12.0761 10.3827C12 10.1989 12 9.96594 12 9.5V7.5Z" stroke="currentColor" stroke-width="1.5" />
        <path d="M13.5 22C19.5 22 21 17.49 21 12C21 6.50998 19.5 2 13.5 2C7.49993 2 6 6.50996 6 12C6 17.49 7.49993 22 13.5 22Z" stroke="currentColor" stroke-width="1.5" />
        <path d="M5 2C3.94531 3.13158 3.23544 4.50113 3 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
      </svg>
      <span>Click here reset</span>
    </button>
  {/if}
</div>
