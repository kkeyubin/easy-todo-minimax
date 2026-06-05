<script setup lang="ts">
import { ref, watch, computed, onUnmounted } from "vue";
import MarkdownIt from "markdown-it";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    fontSize?: number;
    placeholder?: string;
  }>(),
  { fontSize: 14, placeholder: "随手记点什么…" }
);

const emit = defineEmits<{ "update:modelValue": [string] }>();

const md = new MarkdownIt({
  html: false,
  linkify: true,
  breaks: true,
});

const renderedHtml = ref("");

const fontSizePx = computed(() => props.fontSize);

// 防抖渲染：300ms 后才更新渲染层
let renderTimer: number | null = null;
function scheduleRender(text: string) {
  if (renderTimer) clearTimeout(renderTimer);
  renderTimer = window.setTimeout(() => {
    renderedHtml.value = md.render(text || "");
  }, 300);
}

// 立即渲染（程序设置时）
function renderNow(text: string) {
  if (renderTimer) {
    clearTimeout(renderTimer);
    renderTimer = null;
  }
  renderedHtml.value = md.render(text || "");
}

function onInput(e: Event) {
  const v = (e.target as HTMLTextAreaElement).value;
  emit("update:modelValue", v);
  scheduleRender(v);
}

watch(
  () => props.modelValue,
  (v) => {
    if (v) scheduleRender(v);
    else renderNow("");
  },
  { immediate: true }
);

// scroll 同步：textarea scrollTop → 渲染层 scrollTop
const taRef = ref<HTMLTextAreaElement | null>(null);
const renderRef = ref<HTMLElement | null>(null);
function onScroll() {
  if (taRef.value && renderRef.value) {
    renderRef.value.scrollTop = taRef.value.scrollTop;
  }
}

// 监听外部 modelValue 长度大幅变化（程序重置）→ 重新计算高度
watch(
  () => props.modelValue,
  () => {
    if (taRef.value) taRef.value.scrollTop = 0;
  }
);

onUnmounted(() => {
  if (renderTimer) clearTimeout(renderTimer);
});
</script>

<template>
  <div class="text-editor" :style="{ fontSize: fontSizePx + 'px' }">
    <div
      class="text-editor__render"
      ref="renderRef"
      v-html="renderedHtml"
    ></div>
    <textarea
      class="text-editor__textarea"
      ref="taRef"
      :value="modelValue"
      @input="onInput"
      @scroll="onScroll"
      :placeholder="placeholder"
      spellcheck="false"
      autocomplete="off"
      autocorrect="off"
      autocapitalize="off"
    />
  </div>
</template>

<style scoped>
.text-editor {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.text-editor__render,
.text-editor__textarea {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 12px 14px;
  border: 0;
  font-family: inherit;
  font-size: inherit;
  line-height: 1.55;
  background: transparent;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-y: auto;
  box-sizing: border-box;
  scrollbar-gutter: stable;
}

.text-editor__render {
  pointer-events: none;
  color: inherit;
  z-index: 1;
}

.text-editor__textarea {
  color: transparent;
  caret-color: currentColor;
  resize: none;
  outline: none;
  font-family: -apple-system, BlinkMacSystemFont, "PingFang SC", "Helvetica Neue", sans-serif;
  z-index: 2;
  user-select: text;
}
.text-editor__textarea::placeholder {
  color: rgba(0, 0, 0, 0.25);
  user-select: none;
}
.text-editor__textarea::selection {
  background: rgba(0, 100, 200, 0.25);
}

.text-editor__render :deep(p),
.text-editor__render :deep(h1),
.text-editor__render :deep(h2),
.text-editor__render :deep(h3),
.text-editor__render :deep(h4),
.text-editor__render :deep(ul),
.text-editor__render :deep(ol),
.text-editor__render :deep(blockquote),
.text-editor__render :deep(pre) {
  margin: 0 0 8px;
}
.text-editor__render :deep(p:last-child),
.text-editor__render :deep(ul:last-child),
.text-editor__render :deep(ol:last-child) {
  margin-bottom: 0;
}

.text-editor__render :deep(h1) {
  font-size: 1.4em;
  font-weight: 600;
}
.text-editor__render :deep(h2) {
  font-size: 1.2em;
  font-weight: 600;
}
.text-editor__render :deep(h3) {
  font-size: 1.1em;
  font-weight: 600;
}

.text-editor__render :deep(ul),
.text-editor__render :deep(ol) {
  padding-left: 1.5em;
}

.text-editor__render :deep(code) {
  background: rgba(0, 0, 0, 0.07);
  padding: 1px 5px;
  border-radius: 3px;
  font-family: ui-monospace, "SF Mono", Menlo, monospace;
  font-size: 0.92em;
}
.text-editor__render :deep(pre) {
  background: rgba(0, 0, 0, 0.06);
  padding: 8px 10px;
  border-radius: 6px;
  overflow-x: auto;
  font-family: ui-monospace, "SF Mono", Menlo, monospace;
  font-size: 0.92em;
}
.text-editor__render :deep(pre code) {
  background: transparent;
  padding: 0;
  font-size: inherit;
}

.text-editor__render :deep(blockquote) {
  border-left: 3px solid currentColor;
  padding-left: 10px;
  opacity: 0.7;
  margin-left: 0;
}

.text-editor__render :deep(a) {
  color: #2563eb;
  text-decoration: underline;
  text-underline-offset: 2px;
}
.text-editor__render :deep(strong) {
  font-weight: 700;
}
.text-editor__render :deep(em) {
  font-style: italic;
}
.text-editor__render :deep(hr) {
  border: 0;
  border-top: 1px solid rgba(0, 0, 0, 0.12);
  margin: 12px 0;
}
.text-editor__render :deep(input[type="checkbox"]) {
  margin-right: 6px;
  transform: translateY(1px);
}
</style>
