export type StickyType = "text" | "todo" | "link" | "image";

export interface StickyColor {
  id: string;
  bg: string;
  border: string;
  /** 标题栏与工具栏文字用的前景色 */
  fg: string;
}

export const STICKY_COLORS: StickyColor[] = [
  { id: "yellow", bg: "#fff3a3", border: "#e6d770", fg: "#5a4a00" },
  { id: "pink",   bg: "#ffd1dc", border: "#e8a8b8", fg: "#5a2030" },
  { id: "red",    bg: "#ffb3a7", border: "#e89080", fg: "#5a1a10" },
  { id: "blue",   bg: "#b3d9ff", border: "#88b8e8", fg: "#0d3a66" },
  { id: "green",  bg: "#b8e6b8", border: "#90c890", fg: "#1a4a1a" },
  { id: "purple", bg: "#d4b3ff", border: "#b08ce8", fg: "#3a1a66" },
  { id: "orange", bg: "#ffcc99", border: "#e8a868", fg: "#5a3a10" },
  { id: "gray",   bg: "#e0e0e0", border: "#b8b8b8", fg: "#303030" },
];

export const FONT_SIZES = [12, 14, 16, 18, 20] as const;
export type FontSize = (typeof FONT_SIZES)[number];

export function getColor(id: string): StickyColor {
  return STICKY_COLORS.find((c) => c.id === id) ?? STICKY_COLORS[0];
}
