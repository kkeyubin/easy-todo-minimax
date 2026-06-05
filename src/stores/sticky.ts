import { defineStore } from "pinia";
import { ref } from "vue";
import { ipc, type Sticky, type StickyType } from "../ipc";

// W1 阶段 store 主要是给 W2+ 的便签列表 / 全局操作打基础。
// 启动恢复是主进程 WindowManager 负责的，不在 store 里重复。
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
    stickies.value.push(s);
    return s;
  }

  async function remove(id: number) {
    await ipc.delete(id);
    stickies.value = stickies.value.filter((s) => s.id !== id);
  }

  return { stickies, loading, load, create, remove };
});
