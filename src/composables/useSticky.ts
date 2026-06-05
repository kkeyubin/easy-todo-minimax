import { ref, watch, type Ref } from "vue";
import { ipc, type Sticky, type StickyPatch } from "../ipc";

/**
 * 加载并自动持久化便签数据。
 * - load() 拉初始数据
 * - watch(sticky) 防抖 500ms 自动保存
 * - 直接修改 sticky.value.* 即可触发保存
 */
export function useSticky(id: Ref<number>) {
  const sticky = ref<Sticky | null>(null);
  const ready = ref(false);

  async function load() {
    sticky.value = await ipc.get(id.value);
    ready.value = true;
  }

  // 防抖保存：500ms 内合并多次修改
  let saveTimer: number | null = null;
  function scheduleSave() {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = window.setTimeout(async () => {
      if (!sticky.value) return;
      try {
        const patch: StickyPatch = {
          title: sticky.value.title,
          color: sticky.value.color,
          content: sticky.value.content,
          font_size: sticky.value.font_size,
          pinned: sticky.value.pinned,
        };
        await ipc.update(sticky.value.id, patch);
      } catch (e) {
        console.error("[useSticky] save failed", e);
      }
    }, 500);
  }

  watch(
    sticky,
    () => {
      if (ready.value) scheduleSave();
    },
    { deep: true }
  );

  return { sticky, ready, load };
}
