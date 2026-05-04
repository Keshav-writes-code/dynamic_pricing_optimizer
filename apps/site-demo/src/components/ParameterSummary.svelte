<script lang="ts">
  import NumberFlow from "@number-flow/svelte";

  let { 
    demandLevel = 0.8,
    competitorPrice = 45,
    ourPrice = 59,
    dayContext = 'Weekend',
    timeContext = 'Peak Hours'
  } = $props<{ 
    demandLevel?: number;
    competitorPrice?: number;
    ourPrice?: number;
    dayContext?: string;
    timeContext?: string;
  }>();

  let demandText = $derived(demandLevel >= 0.7 ? 'High' : demandLevel >= 0.4 ? 'Medium' : 'Low');
</script>

<section class="flex flex-col gap-3">
  <h3 class="font-display font-semibold text-lg text-base-content px-2 mb-1">Current Parameters</h3>
  
  <!-- Row 1: Demand -->
  <div class="bg-base-200/60 backdrop-blur-md rounded-lg h-14 px-4 flex items-center justify-between shadow-sm">
    <div class="flex items-center gap-3">
      <div class="flex items-center justify-center size-8 rounded-full bg-base-300 text-base-content">
        <span class="i-tabler:trending-up text-[18px]"></span>
      </div>
      <span class="font-medium text-base text-base-content">Demand Level</span>
    </div>
    <span class="font-semibold font-display text-primary">{demandText}</span>
  </div>

  <!-- Row 2: Competitor Price -->
  <div class="bg-base-200/60 backdrop-blur-md rounded-lg h-14 px-4 flex items-center justify-between shadow-sm">
    <div class="flex items-center gap-3">
      <div class="flex items-center justify-center size-8 rounded-full bg-base-300 text-base-content">
        <span class="i-tabler:building-store text-[18px]"></span>
      </div>
      <span class="font-medium text-base text-base-content">Competitor Price</span>
    </div>
    <span class="font-semibold font-display text-base-content flex items-center">₹<NumberFlow value={competitorPrice} format={{ minimumFractionDigits: 2, maximumFractionDigits: 2 }} /></span>
  </div>

  <!-- Row 3: Our Price -->
  <div class="bg-base-200/60 backdrop-blur-md rounded-lg h-14 px-4 flex items-center justify-between shadow-sm">
    <div class="flex items-center gap-3">
      <div class="flex items-center justify-center size-8 rounded-full bg-base-300 text-base-content">
        <span class="i-tabler:tag text-[18px]"></span>
      </div>
      <span class="font-medium text-base text-base-content">Our Price</span>
    </div>
    <span class="font-semibold font-display text-base-content flex items-center">₹<NumberFlow value={ourPrice} format={{ minimumFractionDigits: 2, maximumFractionDigits: 2 }} /></span>
  </div>

  <!-- Row 4: Day Type -->
  <div class="bg-base-200/60 backdrop-blur-md rounded-lg h-14 px-4 flex items-center justify-between shadow-sm">
    <div class="flex items-center gap-3">
      <div class="flex items-center justify-center size-8 rounded-full bg-base-300 text-base-content">
        <span class="i-tabler:calendar text-[18px]"></span>
      </div>
      <span class="font-medium text-base text-base-content">Day Type</span>
    </div>
    <span class="font-semibold font-display text-base-content/70">{dayContext}</span>
  </div>

  <!-- Row 5: Time Factor -->
  <div class="bg-base-200/60 backdrop-blur-md rounded-lg h-14 px-4 flex items-center justify-between shadow-sm">
    <div class="flex items-center gap-3">
      <div class="flex items-center justify-center size-8 rounded-full bg-base-300 text-base-content">
        <span class="i-tabler:clock text-[18px]"></span>
      </div>
      <span class="font-medium text-base text-base-content">Time Factor</span>
    </div>
    <span class="font-semibold font-display text-base-content/70">{timeContext}</span>
  </div>
</section>