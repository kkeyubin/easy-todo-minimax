<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useStickyStore } from "../stores/sticky";
import { getColor } from "../types/sticky";
import type { StickyType } from "../ipc";

const store = useStickyStore();

onMounted(store.load);

const total = computed(() => store.stickies.length);

// cascade 位置：基于最后一个便签 +30,+30，超出屏幕 wrap 回 (200, 200)
function nextPosition(): { x: number; y: number } {
  if (store.stickies.length === 0) return { x: 200, y: 200 };
  const last = store.stickies[0]; // unshift 后第一个是最新
  const newX = last.x + 30;
  const newY = last.y + 30;
  // 简单 wrap：超过 1000 回到起点
  if (newX > 1000 || newY > 800) return { x: 200, y: 200 };
  return { x: newX, y: newY };
}

async function newSticky(type: StickyType) {
  const { x, y } = nextPosition();
  const s = await store.create(type, x, y);
  await store.show(s.id);
}

async function openSticky(id: number) {
  await store.show(id);
}

async function deleteSticky(id: number) {
  if (!confirm("确定删除这个便签？")) return;
  await store.remove(id);
}

function formatTime(iso: string): string {
  try {
    const d = new Date(iso);
    const now = new Date();
    const diff = (now.getTime() - d.getTime()) / 1000;
    if (diff < 60) return "刚刚";
    if (diff < 3600) return `${Math.floor(diff / 60)} 分钟前`;
    if (diff < 86400) return `${Math.floor(diff / 3600)} 小时前`;
    return `${Math.floor(diff / 86400)} 天前`;
  } catch {
    return iso;
  }
}

function typeLabel(t: StickyType): string {
  return { text: "文本", todo: "待办", link: "链接", image: "图片" }[t];
}
</script>

<template>
  <div class="main-window">
    <header class="main-window__header">
      <h1>Easy Sticky</h1>
      <p class="main-window__subtitle">共 {{ total }} 个便签</p>
    </header>

    <main class="main-window__list">
      <div v-if="store.loading" class="main-window__empty">加载中…</div>

      <div v-else-if="!store.stickies.length" class="main-window__empty">
        <p>还没有便签</p>
        <p class="main-window__hint">点下方按钮新建一个</p>
      </div>

      <ul v-else class="sticky-list">
        <li
          v-for="s in store.stickies"
          :key="s.id"
          class="sticky-item"
          @dblclick="openSticky(s.id)"
        >
          <span
            class="sticky-item__color"
            :style="{ background: getColor(s.color).bg, borderColor: getColor(s.color).border }"
          ></span>

          <div class="sticky-item__info">
            <div class="sticky-item__title">
              {{ s.title || "(无标题)" }}
            </div>
            <div class="sticky-item__meta">
              <span class="sticky-item__type" :class="`sticky-item__type--${s.type}`">
                {{ typeLabel(s.type) }}
              </span>
              <span class="sticky-item__time">{{ formatTime(s.created_at) }}</span>
            </div>
          </div>

          <div class="sticky-item__actions">
            <button class="sticky-item__btn" @click="openSticky(s.id)">打开</button>
            <button
              class="sticky-item__btn sticky-item__btn--danger"
              @click="deleteSticky(s.id)"
            >删除</button>
          </div>
        </li>
      </ul>
    </main>

    <footer class="main-window__footer">
      <div class="main-window__new-group">
        <button class="main-window__new" @click="newSticky('text')">
          + 文本
        </button>
        <button class="main-window__new main-window__new--todo" @click="newSticky('todo')">
          + 待办
        </button>
        <button class="main-window__new main-window__new--link" @click="newSticky('link')">
          + 链接
        </button>
      </div>
      <p class="main-window__hint main-window__hint--footer">
        W4.2 还会加图片便签
      </p>
    </footer>
  </div>
</template>

<style scoped>
.main-window {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #fafafa;
  font-family: -apple-system, BlinkMacSystemFont, "PingFang SC", "Helvetica Neue", sans-serif;
}

.main-window__header {
  flex: 0 0 auto;
  padding: 20px 24px 16px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  background: #fff;
}
.main-window__header h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #222;
}
.main-window__subtitle {
  margin: 4px 0 0;
  font-size: 12px;
  color: #888;
}

.main-window__list {
  flex: 1;
  overflow-y: auto;
  padding: 12px 16px;
}

.main-window__empty {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #999;
  font-size: 14px;
}
.main-window__empty p {
  margin: 4px 0;
}
.main-window__hint {
  font-size: 12px;
  color: #aaa;
}

.sticky-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.sticky-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background: #fff;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.06);
  cursor: default;
  transition: border-color 0.1s, box-shadow 0.1s;
}
.sticky-item:hover {
  border-color: rgba(0, 0, 0, 0.12);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.sticky-item__color {
  flex: 0 0 16px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  border: 1.5px solid;
}

.sticky-item__info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.sticky-item__title {
  font-size: 14px;
  font-weight: 500;
  color: #222;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sticky-item__meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: #999;
}

.sticky-item__type {
  padding: 1px 6px;
  border-radius: 3px;
  font-size: 10px;
  font-weight: 500;
  background: #e5e7eb;
  color: #4b5563;
}
.sticky-item__type--text { background: #dbeafe; color: #1e40af; }
.sticky-item__type--todo { background: #fef3c7; color: #92400e; }
.sticky-item__type--link { background: #d1fae5; color: #065f46; }
.sticky-item__type--image { background: #fce7f3; color: #9d174d; }

.sticky-item__actions {
  display: flex;
  gap: 6px;
}
.sticky-item__btn {
  padding: 4px 10px;
  font-size: 12px;
  background: #f3f4f6;
  border: 1px solid rgba(0, 0, 0, 0.08);
  border-radius: 5px;
  cursor: pointer;
  color: #333;
  transition: background 0.1s;
}
.sticky-item__btn:hover {
  background: #e5e7eb;
}
.sticky-item__btn--danger:hover {
  background: #fee2e2;
  color: #dc2626;
  border-color: #fca5a5;
}

.main-window__footer {
  flex: 0 0 auto;
  padding: 16px 24px 20px;
  border-top: 1px solid rgba(0, 0, 0, 0.06);
  background: #fff;
}
.main-window__new-group {
  display: flex;
  gap: 8px;
}
.main-window__new {
  flex: 1;
  padding: 10px 16px;
  font-size: 14px;
  font-weight: 500;
  background: #2563eb;
  color: #fff;
  border: 0;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.1s;
}
.main-window__new--todo {
  background: #f59e0b;
}
.main-window__new--todo:hover {
  background: #d97706;
}
.main-window__new--link {
  background: #10b981;
}
.main-window__new--link:hover {
  background: #059669;
}
.main-window__new:hover {
  background: #1d4ed8;
}
.main-window__new:active {
  background: #1e40af;
}
.main-window__hint--footer {
  margin: 8px 0 0;
  text-align: center;
}
</style>
