<script setup lang="ts">
import { computed, ref } from "vue";
import { useResizable } from "../composables/useResizable";
import { getColor, type StickyColor } from "../types/sticky";
import type { Sticky } from "../ipc";

const props = defineProps<{
  sticky: Sticky;
}>();

const color = computed<StickyColor>(() => getColor(props.sticky.color));
const idRef = computed(() => props.sticky.id);
const { onResizeStart } = useResizable(idRef);

const titleInput = ref<HTMLInputElement | null>(null);

function onTitleInput(e: Event) {
  const v = (e.target as HTMLInputElement).value;
  props.sticky.title = v;
}

function onTitleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter") {
    e.preventDefault();
    titleInput.value?.blur();
  }
}
</script>

<template>
  <div
    class="sticky-card"
    :style="{
      background: color.bg,
      borderColor: color.border,
      color: color.fg,
    }"
  >
    <!-- 标题栏：OS 级 drag region 整个条 -->
    <header
      class="sticky-card__title"
      data-tauri-drag-region
    >
      <input
        ref="titleInput"
        class="sticky-card__title-input"
        :value="sticky.title"
        @input="onTitleInput"
        @keydown="onTitleKeydown"
        placeholder="便签"
        spellcheck="false"
        data-tauri-drag-region
      />
    </header>

    <!-- 内容 slot -->
    <main class="sticky-card__body">
      <slot />
    </main>

    <!-- 底部 toolbar slot（可选） -->
    <slot name="toolbar" />

    <!-- 8 向 resize 手柄 -->
    <div class="sticky-card__resize sticky-card__resize--n"  @mousedown="onResizeStart('n')"></div>
    <div class="sticky-card__resize sticky-card__resize--s"  @mousedown="onResizeStart('s')"></div>
    <div class="sticky-card__resize sticky-card__resize--w"  @mousedown="onResizeStart('w')"></div>
    <div class="sticky-card__resize sticky-card__resize--e"  @mousedown="onResizeStart('e')"></div>
    <div class="sticky-card__resize sticky-card__resize--ne" @mousedown="onResizeStart('ne')"></div>
    <div class="sticky-card__resize sticky-card__resize--nw" @mousedown="onResizeStart('nw')"></div>
    <div class="sticky-card__resize sticky-card__resize--se" @mousedown="onResizeStart('se')"></div>
    <div class="sticky-card__resize sticky-card__resize--sw" @mousedown="onResizeStart('sw')"></div>
  </div>
</template>

<style scoped>
.sticky-card {
  position: relative;
  width: 100vw;
  height: 100vh;
  border-radius: 12px;
  border: 1px solid;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, "PingFang SC", sans-serif;
  user-select: none;
}

.sticky-card__title {
  flex: 0 0 32px;
  display: flex;
  align-items: center;
  padding: 0 12px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  cursor: grab;
}
.sticky-card__title:active {
  cursor: grabbing;
}

.sticky-card__title-input {
  flex: 1;
  background: transparent;
  border: 0;
  outline: 0;
  color: inherit;
  font-size: 12px;
  font-weight: 500;
  padding: 0;
  letter-spacing: 0.02em;
  user-select: text;
  cursor: text;
}
.sticky-card__title-input::placeholder {
  opacity: 0.4;
}

.sticky-card__body {
  flex: 1;
  position: relative;
  overflow: hidden;
}

/* 8 向 resize 手柄：4 边 + 4 角各 8px */
.sticky-card__resize {
  position: absolute;
  z-index: 10;
}
.sticky-card__resize--n  { top: -4px;    left: 8px;    right: 8px;    height: 8px; cursor: ns-resize; }
.sticky-card__resize--s  { bottom: -4px; left: 8px;    right: 8px;    height: 8px; cursor: ns-resize; }
.sticky-card__resize--w  { left: -4px;   top: 8px;     bottom: 8px;   width: 8px;  cursor: ew-resize; }
.sticky-card__resize--e  { right: -4px;  top: 8px;     bottom: 8px;   width: 8px;  cursor: ew-resize; }
.sticky-card__resize--ne { top: -4px;    right: -4px; width: 12px;   height: 12px; cursor: nesw-resize; }
.sticky-card__resize--nw { top: -4px;    left: -4px;  width: 12px;   height: 12px; cursor: nwse-resize; }
.sticky-card__resize--se { bottom: -4px; right: -4px; width: 12px;   height: 12px; cursor: nwse-resize; }
.sticky-card__resize--sw { bottom: -4px; left: -4px;  width: 12px;   height: 12px; cursor: nesw-resize; }
</style>
