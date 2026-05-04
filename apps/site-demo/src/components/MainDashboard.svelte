<script lang="ts">
  import MainNumberCard from './MainNumberCard.svelte';
  import ParameterSummary from './ParameterSummary.svelte';

  let { 
    confidence = 82, 
    demandLevel = 0.8, 
    competitorPrice = 45, 
    ourPrice = 59, 
    dayContext = 'Weekend', 
    timeContext = 'Peak Hours',
    onTweak
  } = $props<{
    confidence?: number;
    demandLevel?: number;
    competitorPrice?: number;
    ourPrice?: number;
    dayContext?: string;
    timeContext?: string;
    onTweak: () => void;
  }>();
</script>

<div class="relative flex h-auto w-full flex-col group/design-root overflow-x-hidden font-display mb-2">
  <div class="flex items-center p-4 pb-2 justify-between bg-transparent">
    <h2 class="text-base-content text-lg font-bold leading-tight tracking-[-0.015em] flex-1 text-center px-12">
      Dashboard
    </h2>
  </div>
</div>

<main class="px-4 pb-32 flex flex-col gap-6 relative z-10">
  <MainNumberCard {confidence} />
  <ParameterSummary {demandLevel} {competitorPrice} {ourPrice} {dayContext} {timeContext} />
</main>

<div class="fixed bottom-6 left-0 right-0 flex justify-center z-40 pointer-events-none px-4">
  <button 
    onclick={onTweak}
    class="pointer-events-auto btn btn-primary rounded-full shadow-lg shadow-primary/20 w-[200px] h-14 font-display font-semibold text-[15px] flex items-center justify-center gap-2 active:scale-95 transition-all"
  >
    <span class="i-tabler:adjustments text-[20px]"></span>
    Tweak Variables
  </button>
</div>
