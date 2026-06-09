<script lang="ts">
  export let text: string = "Release to cancel";
</script>

<div class="wheel-center">
  <span class="center-text">{text}</span>
</div>

<style>
  .wheel-center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: rgba(32, 32, 32, 0.85);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
    z-index: 10;
  }

  .center-text {
    color: rgba(255, 255, 255, 0.7);
    font-size: 10px;
    font-family: 'Segoe UI', system-ui, sans-serif;
    text-align: center;
    line-height: 1.3;
    white-space: nowrap;
  }
</style>
