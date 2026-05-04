<script lang="ts">
  // No props needed
</script>

<div class="fixed inset-0 min-h-screen w-full flex flex-col items-center justify-center font-body text-base-content overflow-hidden relative z-50 bg-base-100/90 backdrop-blur-sm">
  <!-- Processing Orb Container -->
  <div class="relative flex flex-col items-center justify-center gap-8">
    <!-- Ethereal Orb Layer -->
    <div class="relative w-32 h-32 flex items-center justify-center">
      <!-- Outer pulsing blur -->
      <div class="absolute inset-0 bg-primary rounded-full animate-breathe-orb opacity-30"></div>
      <!-- Inner solid core -->
      <div class="absolute w-20 h-20 bg-primary rounded-full shadow-[0_0_32px_rgba(var(--color-primary),0.5)] z-10"></div>
    </div>
    
    <!-- Muted Fading Text -->
    <p class="font-display text-base-content/70 text-base font-semibold tracking-wide animate-pulse-text mt-6">
      Running pricing models...
    </p>
  </div>
</div>

<style>
  @keyframes breathe {
    0%, 100% { transform: scale(1); filter: blur(8px); opacity: 0.3; }
    50% { transform: scale(1.15); filter: blur(16px); opacity: 0.1; }
  }
  @keyframes pulseText {
    0%, 100% { opacity: 0.5; }
    50% { opacity: 1; }
  }
  .animate-breathe-orb {
    animation: breathe 3s ease-in-out infinite;
  }
  .animate-pulse-text {
    animation: pulseText 2s ease-in-out infinite;
  }
</style>
