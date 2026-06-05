import { defineStore } from "pinia";
import { ref } from "vue";
import { ipc, type Sticky, type StickyType } from "../ipc";

/**
 * W2.3: 便签全局 store（main 窗口的列表管理用，便签窗口不依赖这个）
 */
export const useStickyStore = defineStore("sticky", () => {
  const stickies = ref<Sticky[]>([]);
  const loading = ref(false);

  async function load() {
    loading.value = true;
    try {
      stickies.value = await ipc.list();
    } finally {
      loading.value = false;
    }
  }

  async function create(type: StickyType, x: number, y: number) {
    const s = await ipc.create(type, x, y);
    stickies.value.unshift(s);
    return s;
  }

  async function remove(id: number) {
    await ipc.delete(id);
    stickies.value = stickies.value.filter((s) => s.id !== id);
  }

  async function show(id: number) {
    await ipc.showSticky(id);
  }

  return { stickies, loading, load, create, remove, show };
});
