<script lang="ts">
  import NumberFlow from "@number-flow/svelte";

  let { confidence = 82 } = $props<{ confidence?: number }>();
  
  let prev_value = $state(82);
  let direction = $state(0);

  // Update direction when confidence changes
  $effect(() => {
    if (confidence > prev_value) {
      direction = 1;
    } else if (confidence < prev_value) {
      direction = -1;
    }
    prev_value = confidence;
  });

  let badgeInfo = $derived.by(() => {
    if (confidence >= 75) {
      return { text: 'High Margin Advantage', icon: 'i-tabler:trending-up', color: 'text-success' };
    } else if (confidence >= 40) {
      return { text: 'Competitive Pricing', icon: 'i-tabler:scale', color: 'text-warning' };
    } else {
      return { text: 'Pricing Risky: Too high', icon: 'i-tabler:alert-triangle', color: 'text-error' };
    }
  });
</script>

<section class="rounded-2xl flex flex-col items-center justify-center p-6 relative overflow-hidden bg-base-200/50 backdrop-blur-md shadow-xl">
  <span class="text-base-content/70 font-body font-medium text-sm tracking-wider uppercase mb-6 relative z-10">
    Sales Confidence
  </span>
  
  <div
    class="radial-progress {badgeInfo.color} font-display font-bold text-5xl bg-base-300 border-4 border-base-300 transition-colors duration-300"
    style="--value:{confidence}; --size:14rem; --thickness: 1rem;"
    aria-valuenow={confidence}
    role="progressbar"
  >
    <NumberFlow
      value={confidence / 100}
      format={{
        style: "percent",
      }}
    />
  </div>
  
  <div class="mt-8 flex items-center gap-2 bg-base-100/50 px-3 py-1.5 rounded-full border border-base-content/10 relative z-10 backdrop-blur-md">
    <span class="{badgeInfo.icon} {badgeInfo.color} text-base transition-colors duration-300"></span>
    <span class="text-xs font-medium text-base-content transition-colors duration-300">{badgeInfo.text}</span>
  </div>
</section>
