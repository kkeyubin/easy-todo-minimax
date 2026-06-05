import {
  getCurrentWindow,
  PhysicalSize,
  PhysicalPosition,
} from "@tauri-apps/api/window";
import type { Ref } from "vue";
import { ipc } from "../ipc";

/**
 * 8 向 resize：4 边 + 4 角手柄的 mousedown 处理。
 * 监听 mousemove 调 win.setSize / setPosition，mouseup 后防抖同步到 DB。
 */
export function useResizable(stickyId: Ref<number>) {
  const win = getCurrentWindow();
  const MIN = 180;

  function onResizeStart(direction: "n" | "s" | "e" | "w" | "ne" | "nw" | "se" | "sw") {
    return async (e: MouseEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const startMouseX = e.screenX;
      const startMouseY = e.screenY;
      const startSize = await win.outerSize();
      const startPos = await win.outerPosition();
      const startW = startSize.width;
      const startH = startSize.height;
      const startX = startPos.x;
      const startY = startPos.y;

      let curX = startMouseX;
      let curY = startMouseY;
      let lastApplied = 0;

      async function apply() {
        const dx = curX - startMouseX;
        const dy = curY - startMouseY;

        let newW = startW;
        let newH = startH;
        let newX = startX;
        let newY = startY;

        if (direction.includes("e")) newW = Math.max(MIN, startW + dx);
        if (direction.includes("w")) {
          newW = Math.max(MIN, startW - dx);
          newX = startX + (startW - newW);
        }
        if (direction.includes("s")) newH = Math.max(MIN, startH + dy);
        if (direction.includes("n")) {
          newH = Math.max(MIN, startH - dy);
          newY = startY + (startH - newH);
        }

        await win.setSize(new PhysicalSize(newW, newH));
        await win.setPosition(new PhysicalPosition(newX, newY));
      }

      function onMove(ev: MouseEvent) {
        curX = ev.screenX;
        curY = ev.screenY;
        const now = performance.now();
        // ~60fps 节流
        if (now - lastApplied >= 16) {
          lastApplied = now;
          apply();
        }
      }

      let saveTimer: number | null = null;
      function onUp() {
        document.removeEventListener("mousemove", onMove);
        document.removeEventListener("mouseup", onUp);
        // 最后一帧同步
        apply();
        // 防抖保存到 DB
        if (saveTimer) clearTimeout(saveTimer);
        saveTimer = window.setTimeout(async () => {
          try {
            const size = await win.outerSize();
            const pos = await win.outerPosition();
            await ipc.patchWindowState(
              stickyId.value,
              pos.x,
              pos.y,
              size.width,
              size.height
            );
          } catch (e) {
            console.error("[useResizable] save failed", e);
          }
        }, 500);
      }

      document.addEventListener("mousemove", onMove);
      document.addEventListener("mouseup", onUp);
    };
  }

  return { onResizeStart };
}
