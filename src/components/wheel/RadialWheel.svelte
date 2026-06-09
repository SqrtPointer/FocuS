<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import WheelCanvas from "./WheelCanvas.svelte";
  import WheelOverlay from "./WheelOverlay.svelte";
  import WheelCenter from "./WheelCenter.svelte";
  import { activeWheelSector, wheelItems, appConfig } from "../../lib/stores";
  import { hideWheel, launchApp, getConfig } from "../../lib/commands";
  import { getActiveSector, DEAD_ZONE_RADIUS, WHEEL_RADIUS } from "../../lib/wheel";
  import type { WheelItem } from "../../lib/types";

  let canvasW = 400;
  let canvasH = 400;
  let cx = canvasW / 2;
  let cy = canvasH / 2;
  let sector: number | null = null;
  let prevSector: number | null = null;
  let isPressed = true; // Track key hold state

  const defaultWheelItems: WheelItem[] = [
    { sector: 6, title: "VS Code", icon: "💻", action: { type: "LaunchApp", path: "code" } },
    { sector: 7, title: "Browser", icon: "🌐", action: { type: "LaunchApp", path: "msedge" } },
    { sector: 0, title: "Terminal", icon: "⬛", action: { type: "LaunchApp", path: "wt" } },
    { sector: 1, title: "Explorer", icon: "📁", action: { type: "LaunchApp", path: "explorer" } },
    { sector: 2, title: "Calc", icon: "🧮", action: { type: "LaunchApp", path: "calc" } },
    { sector: 3, title: "Snipping", icon: "✂️", action: { type: "LaunchApp", path: "snippingtool" } },
    { sector: 4, title: "Notepad", icon: "📝", action: { type: "LaunchApp", path: "notepad" } },
    { sector: 5, title: "Settings", icon: "⚙️", action: { type: "LaunchApp", path: "ms-settings:" } },
  ];

  onMount(() => {
    // Load wheel config
    getConfig().then(cfg => {
      $appConfig = cfg;
    });
    $wheelItems = defaultWheelItems;

    document.addEventListener("keyup", handleKeyUp);
    document.addEventListener("mousemove", handleMouseMove);
  });

  onDestroy(() => {
    document.removeEventListener("keyup", handleKeyUp);
    document.removeEventListener("mousemove", handleMouseMove);
  });

  function handleKeyUp(e: KeyboardEvent) {
    if (e.code === "Space" && e.ctrlKey) {
      isPressed = false;
      executeAndClose();
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isPressed) return;
    const rect = (e.target as HTMLElement)?.getBoundingClientRect?.();
    if (!rect) {
      // Use clientX/Y relative to center of wheel
      sector = getActiveSector(e.clientX, e.clientY, window.innerWidth / 2, window.innerHeight / 2);
    }
    if (sector !== prevSector) {
      prevSector = sector;
      $activeWheelSector = sector;
    }
  }

  function executeAndClose() {
    if (sector === null) {
      // Dead zone — cancel
      hideWheel();
      return;
    }
    if (sector >= 0) {
      const item = $wheelItems.find(w => w.sector === sector);
      if (item?.action.type === "LaunchApp") {
        launchApp(item.action.path);
      }
    }
    hideWheel();
  }

  function getCenterText(): string {
    if (sector === null) return "Release to cancel";
    if (sector < 0) return "...";
    const item = $wheelItems.find(w => w.sector === sector);
    return item?.title ?? "Unknown";
  }
</script>

<WheelOverlay />
<WheelCanvas
  width={canvasW}
  height={canvasH}
  items={$wheelItems}
  activeSector={sector}
  borderColor={$appConfig?.appearance.wheel.border_color ?? "#60CDFF"}
  borderWidth={$appConfig?.appearance.wheel.border_width ?? 2}
  borderGlow={$appConfig?.appearance.wheel.border_glow ?? 8}
/>
<WheelCenter text={getCenterText()} />
