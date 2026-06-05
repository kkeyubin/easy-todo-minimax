<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import StickyColorPicker from "./StickyColorPicker.vue";
import { getColor, FONT_SIZES } from "../types/sticky";
import { ipc, type Sticky } from "../ipc";

const props = defineProps<{
  sticky: Sticky;
}>();

const colorOpen = ref(false);
const colorPicker = ref<HTMLElement | null>(null);

const currentColor = computed(() => getColor(props.sticky.color));

function onColorChange(id: string) {
  props.sticky.color = id; // useSticky.watch deep 触发自动保存
  colorOpen.value = false;
}

function decFont() {
  const idx = FONT_SIZES.indexOf(props.sticky.font_size as any);
  if (idx > 0) props.sticky.font_size = FONT_SIZES[idx - 1];
}
function incFont() {
  const idx = FONT_SIZES.indexOf(props.sticky.font_size as any);
  if (idx >= 0 && idx < FONT_SIZES.length - 1)
    props.sticky.font_size = FONT_SIZES[idx + 1];
}

async function togglePin() {
  const next = props.sticky.pinned === 1 ? 0 : 1;
  props.sticky.pinned = next;
  try {
    await getCurrentWebviewWindow().setAlwaysOnTop(next === 1);
  } catch (e) {
    console.error("[toolbar] setAlwaysOnTop failed", e);
  }
}

async function onDelete() {
  if (!confirm("确定删除这个便签？")) return;
  await ipc.delete(props.sticky.id);
  // 后端 delete_sticky 会关窗，前端无需再操作
}

// 点 picker 外部关闭
function onDocClick(e: MouseEvent) {
  if (!colorOpen.value) return;
  if (colorPicker.value && !colorPicker.value.contains(e.target as Node)) {
    colorOpen.value = false;
  }
}
onMounted(() => document.addEventListener("mousedown", onDocClick));
onUnmounted(() => document.removeEventListener("mousedown", onDocClick));
</script>

<template>
  <footer class="toolbar" @mousedown.stop>
    <!-- 颜色 -->
    <div class="toolbar__group" ref="colorPicker">
      <button
        class="toolbar__btn toolbar__btn--color"
        :title="`当前颜色：${sticky.color}（点击换色）`"
        @click="colorOpen = !colorOpen"
      >
        <span
          class="toolbar__swatch"
          :style="{ background: currentColor.bg, borderColor: currentColor.border }"
        ></span>
      </button>
      <transition name="popover">
        <StickyColorPicker
          v-if="colorOpen"
          :model-value="sticky.color"
          class="toolbar__popover"
          @update:model-value="onColorChange"
        />
      </transition>
    </div>

    <div class="toolbar__sep" />

    <!-- 字号 -->
    <div class="toolbar__group">
      <button
        class="toolbar__btn toolbar__btn--font"
        :disabled="sticky.font_size <= FONT_SIZES[0]"
        @click="decFont"
        title="缩小"
      >A-</button>
      <span class="toolbar__size">{{ sticky.font_size }}</span>
      <button
        class="toolbar__btn toolbar__btn--font"
        :disabled="sticky.font_size >= FONT_SIZES[FONT_SIZES.length - 1]"
        @click="incFont"
        title="放大"
      >A+</button>
    </div>

    <div class="toolbar__sep" />

    <!-- 置顶 -->
    <button
      class="toolbar__btn"
      :class="{ 'toolbar__btn--active': sticky.pinned === 1 }"
      :title="sticky.pinned ? '取消置顶' : '置顶'"
      @click="togglePin"
    >
      <svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor">
        <path
          v-if="sticky.pinned === 1"
          d="M9.5 1.5a3.5 3.5 0 0 0-3.95 3.05L5.5 5H3.5a.5.5 0 0 0-.35.86l2.4 2.4-2.8 2.8a.5.5 0 0 0 .35.86h2.45l-1.6 3.55a.5.5 0 0 0 .91.41L8 12.13l2.13 3.75a.5.5 0 0 0 .91-.41L9.45 11.92h2.45a.5.5 0 0 0 .35-.86l-2.8-2.8 2.4-2.4a.5.5 0 0 0-.35-.86h-2l-.05-.45A3.5 3.5 0 0 0 9.5 1.5Z"
        />
        <path
          v-else
          d="M9.5 2a3 3 0 0 0-3 3v.5H4.5a.5.5 0 0 0-.35.86l2.4 2.4-2.8 2.8a.5.5 0 0 0 .35.86h2.6l-1.5 3.3a.5.5 0 0 0 .9.45L8 12.55l1.9 3.62a.5.5 0 0 0 .9-.45l-1.5-3.3h2.6a.5.5 0 0 0 .35-.86l-2.8-2.8 2.4-2.4a.5.5 0 0 0-.35-.86H10.5V5a3 3 0 0 0-1-3Z"
          fill="none"
          stroke="currentColor"
          stroke-width="1.2"
        />
      </svg>
    </button>

    <div class="toolbar__spacer" />

    <!-- 删除 -->
    <button
      class="toolbar__btn toolbar__btn--danger"
      title="删除便签"
      @click="onDelete"
    >
      <svg viewBox="0 0 16 16" width="13" height="13" fill="currentColor">
        <path d="M5.5 5.5v6h1v-6h-1Zm2 0v6h1v-6h-1Zm2 0v6h1v-6h-1Z" />
        <path
          d="M14.5 3h-3V1.5a1.5 1.5 0 0 0-1.5-1.5h-4A1.5 1.5 0 0 0 4.5 1.5V3h-3a.5.5 0 0 0 0 1h.59l.93 9.74A1.5 1.5 0 0 0 4.51 15h6.98a1.5 1.5 0 0 0 1.49-1.26L13.91 4h.59a.5.5 0 0 0 0-1ZM5.5 1.5a.5.5 0 0 1 .5-.5h4a.5.5 0 0 1 .5.5V3h-5V1.5Zm6.97 12.04a.5.5 0 0 1-.5.46H4.51a.5.5 0 0 1-.5-.46L3.1 4h9.8l-.43 9.54Z"
        />
      </svg>
    </button>
  </footer>
</template>

<style scoped>
.toolbar {
  flex: 0 0 32px;
  display: flex;
  align-items: center;
  padding: 0 8px;
  border-top: 1px solid rgba(0, 0, 0, 0.06);
  gap: 4px;
  position: relative;
  user-select: none;
}

.toolbar__group {
  display: flex;
  align-items: center;
  gap: 2px;
  position: relative;
}

.toolbar__sep {
  width: 1px;
  height: 16px;
  background: rgba(0, 0, 0, 0.1);
  margin: 0 4px;
}

.toolbar__spacer {
  flex: 1;
}

.toolbar__btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 0;
  color: inherit;
  font-size: 12px;
  font-weight: 500;
  width: 24px;
  height: 22px;
  border-radius: 4px;
  cursor: pointer;
  padding: 0;
  opacity: 0.7;
  transition: background 0.1s, opacity 0.1s;
}
.toolbar__btn:hover:not(:disabled) {
  background: rgba(0, 0, 0, 0.08);
  opacity: 1;
}
.toolbar__btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}
.toolbar__btn--active {
  opacity: 1;
  background: rgba(0, 0, 0, 0.1);
}

.toolbar__btn--color {
  width: 24px;
}
.toolbar__swatch {
  display: inline-block;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  border: 1px solid;
}

.toolbar__btn--font {
  width: 22px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: -0.5px;
}
.toolbar__size {
  font-size: 10px;
  font-weight: 500;
  opacity: 0.6;
  min-width: 18px;
  text-align: center;
}

.toolbar__btn--danger:hover {
  background: rgba(220, 38, 38, 0.15);
  color: #dc2626;
}

.toolbar__popover {
  position: absolute;
  bottom: calc(100% + 6px);
  left: 0;
  z-index: 100;
}

.popover-enter-active,
.popover-leave-active {
  transition: opacity 0.12s, transform 0.12s;
}
.popover-enter-from,
.popover-leave-to {
  opacity: 0;
  transform: translateY(4px);
}
</style>
