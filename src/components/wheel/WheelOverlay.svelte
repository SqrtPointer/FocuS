<script lang="ts">
  // Semi-transparent backdrop for the wheel
</script>

<div class="wheel-overlay"></div>

<style>
  .wheel-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(2px);
    -webkit-backdrop-filter: blur(2px);
    pointer-events: none;
    animation: overlayIn 200ms ease-out;
  }

  @keyframes overlayIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
