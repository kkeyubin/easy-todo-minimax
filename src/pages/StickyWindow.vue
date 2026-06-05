<template>
  <div class="sticky-window">
    <header class="sticky-window__title" data-tauri-drag-region>
      <span class="sticky-window__label">Sticky #{{ id }}</span>
    </header>
    <main class="sticky-window__body">
      <p>W1 骨架：便签已加载，待 W2 接入自绘 / 编辑 / 颜色 / 置顶。</p>
      <p v-if="sticky">type: {{ sticky.type }} · color: {{ sticky.color }} · {{ sticky.width }}×{{ sticky.height }}</p>
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { ipc, type Sticky } from "../ipc";

const route = useRoute();
const id = Number(route.params.id);
const sticky = ref<Sticky | null>(null);

onMounted(async () => {
  try {
    sticky.value = await ipc.get(id);
  } catch (e) {
    console.error("get sticky failed", e);
  }
});
</script>

<style>
.sticky-window {
  height: 100vh;
  display: flex;
  flex-direction: column;
}
.sticky-window__title {
  height: 32px;
  display: flex;
  align-items: center;
  padding: 0 12px;
  background: rgba(0, 0, 0, 0.04);
  font-size: 12px;
  color: #555;
  cursor: grab;
}
.sticky-window__title:active {
  cursor: grabbing;
}
.sticky-window__body {
  flex: 1;
  padding: 12px;
  overflow: auto;
}
</style>
