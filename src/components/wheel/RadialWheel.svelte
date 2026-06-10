<script lang="ts">
  import { onMount } from "svelte";
  import WheelCanvas from "./WheelCanvas.svelte";
  import WheelOverlay from "./WheelOverlay.svelte";
  import WheelCenter from "./WheelCenter.svelte";
  import { activeWheelSector, wheelItems, appConfig } from "../../lib/stores";
  import { hideWheel, launchApp, getConfig } from "../../lib/commands";
  import { getActiveSector } from "../../lib/wheel";
  import type { WheelItem } from "../../lib/types";

  const WHEEL_SIZE = 400;
  let sector: number | null = null;
  let prevSector: number | null = null;

  const defaultWheelItems: WheelItem[] = [
    { sector: 6, title: "VS Code", icon: "\u{1F4BB}", action: { type: "LaunchApp", path: "code" } },
    { sector: 7, title: "Browser", icon: "\u{1F310}", action: { type: "LaunchApp", path: "msedge" } },
    { sector: 0, title: "Terminal", icon: "\u{2B1B}", action: { type: "LaunchApp", path: "wt" } },
    { sector: 1, title: "Explorer", icon: "\u{1F4C1}", action: { type: "LaunchApp", path: "explorer" } },
    { sector: 2, title: "Calc", icon: "\u{1F9EE}", action: { type: "LaunchApp", path: "calc" } },
    { sector: 3, title: "Snipping", icon: "\u{2702}\u{FE0F}", action: { type: "LaunchApp", path: "snippingtool" } },
    { sector: 4, title: "Notepad", icon: "\u{1F4DD}", action: { type: "LaunchApp", path: "notepad" } },
    { sector: 5, title: "Settings", icon: "\u{2699}\u{FE0F}", action: { type: "LaunchApp", path: "ms-settings:" } },
  ];

  onMount(async () => {
    // Load config and wheel items
    try {
      const cfg = await getConfig();
      $appConfig = cfg;
    } catch { /* use defaults */ }
    $wheelItems = defaultWheelItems;

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mousedown", handleMouseDown);
  });

  function handleMouseMove(e: MouseEvent) {
    // Window center is at (WHEEL_SIZE/2, WHEEL_SIZE/2) in local coords
    const cx = WHEEL_SIZE / 2;
    const cy = WHEEL_SIZE / 2;
    sector = getActiveSector(e.clientX, e.clientY, cx, cy);
    if (sector !== prevSector) {
      prevSector = sector;
      $activeWheelSector = sector;
    }
  }

  function handleMouseDown(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();

    const cx = WHEEL_SIZE / 2;
    const cy = WHEEL_SIZE / 2;
    const clicked = getActiveSector(e.clientX, e.clientY, cx, cy);

    if (clicked !== null && clicked >= 0) {
      const item = $wheelItems.find(w => w.sector === clicked);
      if (item?.action.type === "LaunchApp") {
        launchApp(item.action.path);
      }
    }
    hideWheel();
  }

  function getCenterText(): string {
    if (sector === null) return "Click or blur to cancel";
    if (sector < 0) return "...";
    const item = $wheelItems.find(w => w.sector === sector);
    return item?.title ?? "Unknown";
  }
</script>

<WheelOverlay />
<WheelCanvas
  width={WHEEL_SIZE}
  height={WHEEL_SIZE}
  items={$wheelItems}
  activeSector={sector}
  borderColor={$appConfig?.appearance.wheel.border_color ?? "#60CDFF"}
  borderWidth={$appConfig?.appearance.wheel.border_width ?? 2}
  borderGlow={$appConfig?.appearance.wheel.border_glow ?? 8}
/>
<WheelCenter text={getCenterText()} />
