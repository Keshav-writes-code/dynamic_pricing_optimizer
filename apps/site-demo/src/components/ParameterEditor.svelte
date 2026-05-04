<script lang="ts">
  import { fly } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import NumberFlow from "@number-flow/svelte";

  let { 
    ourPrice = $bindable(59),
    competitorPrice = $bindable(45),
    demandLevel = $bindable(0.8),
    dayContext = $bindable('Weekend'),
    timeContext = $bindable('Peak Hours'),
    onClose,
    onRun
  } = $props<{
    ourPrice?: number;
    competitorPrice?: number;
    demandLevel?: number;
    dayContext?: string;
    timeContext?: string;
    onClose: () => void;
    onRun: () => void;
  }>();

  let dragY = $state(0);
  let isDragging = $state(false);
  let startY = 0;

  function handlePointerDown(e: PointerEvent) {
    isDragging = true;
    startY = e.clientY - dragY;
    (e.currentTarget as HTMLElement)?.setPointerCapture(e.pointerId);
  }

  function handlePointerMove(e: PointerEvent) {
    if (!isDragging) return;
    const newY = e.clientY - startY;
    if (newY > 0) {
      dragY = newY;
    }
  }

  function handlePointerUp(e: PointerEvent) {
    if (!isDragging) return;
    isDragging = false;
    if (dragY > 150) {
      onClose();
    } else {
      dragY = 0;
    }
    (e.currentTarget as HTMLElement)?.releasePointerCapture(e.pointerId);
  }
</script>

<div 
  transition:fly={{ y: '100%', duration: 300, easing: cubicOut }}
  class="fixed inset-x-0 bottom-0 flex h-[85vh] w-full flex-col bg-base-100 rounded-t-xl overflow-hidden z-50 shadow-[0_-8px_32px_rgba(0,0,0,0.3)] border-t border-base-content/10 {isDragging ? '' : 'transition-transform duration-300'}"
  style="transform: translateY({dragY}px);"
>
  <!-- Handle Area -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    class="flex flex-col items-stretch pt-2 pb-2 cursor-grab active:cursor-grabbing touch-none"
    onpointerdown={handlePointerDown}
    onpointermove={handlePointerMove}
    onpointerup={handlePointerUp}
    onpointercancel={handlePointerUp}
  >
    <button aria-label="Drag Handle" class="flex h-5 w-full items-center justify-center pointer-events-none">
      <div class="h-1.5 w-12 rounded-full bg-base-content opacity-30"></div>
    </button>
  </div>

  <!-- Header -->
  <div class="flex items-center px-6 pb-4 justify-between border-b border-base-content/10">
    <h2 class="text-base-content text-2xl font-bold leading-tight flex-1 font-display">Adjust Parameters</h2>
    <div class="flex w-12 items-center justify-end">
      <button aria-label="Close" onclick={onClose} class="btn btn-circle btn-ghost btn-sm text-base-content">
        <span class="i-tabler:x text-xl"></span>
      </button>
    </div>
  </div>

  <!-- Scrollable Content -->
  <div class="flex-1 overflow-y-auto px-6 py-6 space-y-8 pb-32">
    <!-- Parameter 1: Our Price -->
    <div class="flex flex-col gap-2">
      <div class="flex justify-between items-end mb-2">
        <label for="our-price-input" class="text-base-content text-base font-semibold">Our Price (₹)</label>
        <span class="text-3xl font-bold font-display text-primary tracking-tight flex items-center">₹<NumberFlow value={ourPrice} /></span>
      </div>
      <input id="our-price-input" type="range" min="0" max="10000" bind:value={ourPrice} class="range range-primary" />
    </div>

    <!-- Parameter 2: Comp Price -->
    <div class="flex flex-col gap-2">
      <div class="flex justify-between items-end mb-2">
        <label for="comp-price-input" class="text-base-content text-base font-semibold">Competitor Price (₹)</label>
        <span class="text-3xl font-bold font-display tracking-tight">₹{competitorPrice}</span>
      </div>
      <input id="comp-price-input" type="range" min="0" max="150" bind:value={competitorPrice} class="range" />
    </div>

    <div class="divider my-0"></div>

    <!-- Segmented Controls -->
    <div class="space-y-6">
      <!-- Day Context -->
      <fieldset class="flex flex-col gap-3">
        <legend class="text-base-content text-base font-semibold">Day Context</legend>
        <div class="flex p-1 bg-base-200 rounded-lg">
          <button type="button" onclick={() => dayContext = 'Weekday'} class="flex-1 py-3 px-4 text-sm rounded-[6px] transition-all text-center {dayContext === 'Weekday' ? 'font-semibold bg-base-100 text-base-content shadow-sm' : 'font-medium text-base-content/60 hover:text-base-content'}">
            Weekday
          </button>
          <button type="button" onclick={() => dayContext = 'Weekend'} class="flex-1 py-3 px-4 text-sm rounded-[6px] transition-all text-center {dayContext === 'Weekend' ? 'font-semibold bg-base-100 text-base-content shadow-sm' : 'font-medium text-base-content/60 hover:text-base-content'}">
            Weekend
          </button>
        </div>
      </fieldset>

      <!-- Time Context -->
      <fieldset class="flex flex-col gap-3">
        <legend class="text-base-content text-base font-semibold">Time Context</legend>
        <div class="flex p-1 bg-base-200 rounded-lg">
          <button type="button" onclick={() => timeContext = 'Off-Peak'} class="flex-1 py-3 px-4 text-sm rounded-[6px] transition-all text-center {timeContext === 'Off-Peak' ? 'font-semibold bg-base-100 text-base-content shadow-sm' : 'font-medium text-base-content/60 hover:text-base-content'}">
            Off-Peak
          </button>
          <button type="button" onclick={() => timeContext = 'Peak Hours'} class="flex-1 py-3 px-4 text-sm rounded-[6px] transition-all text-center {timeContext === 'Peak Hours' ? 'font-semibold bg-base-100 text-base-content shadow-sm' : 'font-medium text-base-content/60 hover:text-base-content'}">
            Peak
          </button>
        </div>
      </fieldset>
    </div>

    <div class="divider my-0"></div>

    <!-- Parameter 3: Demand Level -->
    <div class="flex flex-col gap-2 pb-6">
      <div class="flex justify-between items-end mb-2">
        <label for="demand-input" class="text-base-content text-base font-semibold">Demand Level</label>
        <span class="text-3xl font-bold font-display tracking-tight">{demandLevel.toFixed(1)}</span>
      </div>
      <input id="demand-input" type="range" min="0" max="1" step="0.1" bind:value={demandLevel} class="range" />
      <div class="flex justify-between text-xs text-base-content/60 font-medium mt-1">
        <span>Low (0.0)</span><span>High (1.0)</span>
      </div>
    </div>
  </div>

  <!-- Fixed Bottom Action Area -->
  <div class="absolute bottom-0 left-0 right-0 p-6 bg-base-100/90 backdrop-blur-md border-t border-base-content/10 z-20">
    <button 
      onclick={onRun}
      class="btn btn-primary w-full h-14 font-bold text-lg rounded-lg shadow-lg flex items-center justify-center gap-2"
    >
      <span class="i-tabler:bolt text-[20px]"></span>
      Run Optimizer
    </button>
  </div>
</div>
