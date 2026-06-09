/** Wheel geometry constants */
export const WHEEL_RADIUS = 180;
export const DEAD_ZONE_RADIUS = 40;
export const SECTOR_COUNT = 8;
export const SECTOR_ANGLE = (2 * Math.PI) / SECTOR_COUNT; // 45° in radians

/** Sector labels, ordered starting from right (east), going clockwise */
export const SECTOR_LABELS: Record<number, { direction: string; angle: number }> = {
  0: { direction: "→", angle: 0 },        // Right / East
  1: { direction: "↘", angle: 45 },       // Bottom-Right / Southeast
  2: { direction: "↓", angle: 90 },       // Bottom / South
  3: { direction: "↙", angle: 135 },      // Bottom-Left / Southwest
  4: { direction: "←", angle: 180 },      // Left / West
  5: { direction: "↖", angle: 225 },      // Top-Left / Northwest
  6: { direction: "↑", angle: 270 },      // Top / North
  7: { direction: "↗", angle: 315 },      // Top-Right / Northeast
};

/**
 * Determine which wheel sector the mouse is pointing at.
 * Returns the sector index (0-7), null if in dead zone, or -1 if outside wheel.
 */
export function getActiveSector(
  mouseX: number,
  mouseY: number,
  cx: number,
  cy: number
): number | null {
  const dx = mouseX - cx;
  const dy = mouseY - cy;
  const dist = Math.sqrt(dx * dx + dy * dy);

  // Dead zone — center area, releasing here cancels
  if (dist < DEAD_ZONE_RADIUS) return null;

  // Outside wheel bounds
  if (dist > WHEEL_RADIUS) return -1;

  // Calculate angle (atan2: 0=right, positive=down/clockwise)
  // Offset by half a sector so sectors are centered on cardinal directions
  const angleDeg = (Math.atan2(dy, dx) * 180) / Math.PI;
  const adjusted = (angleDeg + 360 + SECTOR_ANGLE * (180 / Math.PI) / 2) % 360;
  const sector = Math.floor(adjusted / ((360) / SECTOR_COUNT)) % SECTOR_COUNT;
  return sector;
}

/**
 * Canvas rendering: draw the outer ring with glow effect.
 */
export function drawOuterRing(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  radius: number,
  color: string,
  width: number,
  glow: number
): void {
  ctx.save();
  ctx.beginPath();
  ctx.arc(cx, cy, radius, 0, Math.PI * 2);
  ctx.strokeStyle = color;
  ctx.lineWidth = width;
  ctx.shadowColor = color;
  ctx.shadowBlur = glow;
  ctx.stroke();
  ctx.restore();
}

/**
 * Canvas rendering: draw a single sector.
 */
export function drawSector(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  innerRadius: number,
  outerRadius: number,
  startAngle: number,
  endAngle: number,
  fillColor: string,
  isActive: boolean
): void {
  ctx.save();
  ctx.beginPath();
  ctx.moveTo(cx, cy);
  ctx.arc(cx, cy, outerRadius, startAngle, endAngle);
  ctx.lineTo(cx, cy);
  ctx.closePath();

  // Clip to donut shape (subtract inner circle)
  ctx.arc(cx, cy, innerRadius, 0, Math.PI * 2);
  ctx.clip("evenodd");

  ctx.beginPath();
  ctx.arc(cx, cy, outerRadius, startAngle, endAngle);
  ctx.lineTo(cx, cy);
  ctx.closePath();
  ctx.fillStyle = fillColor;

  if (isActive) {
    ctx.fillStyle = "rgba(96, 205, 255, 0.25)";
    ctx.shadowColor = "rgba(96, 205, 255, 0.5)";
    ctx.shadowBlur = 12;
  }

  ctx.fill();
  ctx.restore();
}

/**
 * Canvas rendering: draw center circle.
 */
export function drawCenterCircle(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  radius: number,
  text: string
): void {
  ctx.save();
  ctx.beginPath();
  ctx.arc(cx, cy, radius, 0, Math.PI * 2);
  ctx.fillStyle = "rgba(32, 32, 32, 0.6)";
  ctx.fill();
  ctx.strokeStyle = "rgba(255, 255, 255, 0.1)";
  ctx.lineWidth = 1;
  ctx.stroke();

  // Draw text
  ctx.fillStyle = "rgba(255, 255, 255, 0.7)";
  ctx.font = "12px 'Segoe UI', sans-serif";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText(text, cx, cy);
  ctx.restore();
}
