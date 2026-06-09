<script lang="ts">
  import { onMount } from "svelte";
  import type { WheelItem } from "../../lib/types";
  import {
    drawOuterRing,
    drawSector,
    SECTOR_ANGLE,
    WHEEL_RADIUS,
    DEAD_ZONE_RADIUS,
  } from "../../lib/wheel";

  export let width: number = 400;
  export let height: number = 400;
  export let items: WheelItem[] = [];
  export let activeSector: number | null = null;
  export let borderColor: string = "#60CDFF";
  export let borderWidth: number = 2;
  export let borderGlow: number = 8;

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let cx: number;
  let cy: number;
  let animFrame: number;

  onMount(() => {
    ctx = canvas.getContext("2d");
    cx = width / 2;
    cy = height / 2;
    render();
  });

  // Reactive render when activeSector changes
  $: {
    activeSector;
    borderColor;
    borderWidth;
    borderGlow;
    items;
    if (ctx) render();
  }

  function render() {
    if (!ctx) return;
    const c = ctx;
    c.clearRect(0, 0, width, height);

    // Draw sectors
    for (let i = 0; i < 8; i++) {
      const startAngle = i * SECTOR_ANGLE - Math.PI / 2 - SECTOR_ANGLE / 2;
      const endAngle = startAngle + SECTOR_ANGLE;
      const isActive = activeSector === i;
      drawSector(c, cx, cy, DEAD_ZONE_RADIUS, WHEEL_RADIUS, startAngle, endAngle, "", isActive);

      // Draw icon and label
      const midAngle = startAngle + SECTOR_ANGLE / 2;
      const labelRadius = (DEAD_ZONE_RADIUS + WHEEL_RADIUS) / 2 + 15;
      const lx = cx + Math.cos(midAngle) * labelRadius;
      const ly = cy + Math.sin(midAngle) * labelRadius;

      const item = items.find(w => w.sector === i);
      if (item) {
        // Icon
        c.font = "22px 'Segoe UI', sans-serif";
        c.textAlign = "center";
        c.textBaseline = "middle";
        c.fillStyle = "rgba(255,255,255,0.9)";
        c.fillText(item.icon, lx, ly - 12);

        // Label
        c.font = "10px 'Segoe UI', sans-serif";
        c.fillStyle = "rgba(255,255,255,0.6)";
        c.fillText(item.title, lx, ly + 14);
      }
    }

    // Draw outer ring
    drawOuterRing(c, cx, cy, WHEEL_RADIUS, borderColor, borderWidth, borderGlow);

    // Draw inner ring (dead zone border)
    c.beginPath();
    c.arc(cx, cy, DEAD_ZONE_RADIUS, 0, Math.PI * 2);
    c.strokeStyle = "rgba(255,255,255,0.08)";
    c.lineWidth = 1;
    c.stroke();
  }
</script>

<canvas bind:this={canvas} {width} {height} />

<style>
  canvas {
    position: absolute;
    top: 0;
    left: 0;
  }
</style>
