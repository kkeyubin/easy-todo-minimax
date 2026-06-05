<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { open } from "@tauri-apps/plugin-shell";

const props = defineProps<{
  modelValue: string;
  fontSize?: number;
  placeholder?: string;
}>();

const emit = defineEmits<{ "update:modelValue": [string] }>();

// 自动从输入文本提取第一个 URL
function extractUrl(text: string): string {
  if (!text) return "";
  const m = text.match(/https?:\/\/[^\s<>"']+/i);
  return m ? m[0] : "";
}

const text = ref(props.modelValue ?? "");
const url = ref(extractUrl(text.value));

watch(
  () => props.modelValue,
  (v) => {
    if (v !== text.value) {
      text.value = v ?? "";
      url.value = extractUrl(text.value);
    }
  }
);

function onInput(e: Event) {
  text.value = (e.target as HTMLTextAreaElement).value;
  url.value = extractUrl(text.value);
  emit("update:modelValue", text.value);
}

function onBlur() {
  // blur 时把 url 写回 content（统一存 URL）
  if (url.value && url.value !== text.value) {
    text.value = url.value;
    emit("update:modelValue", text.value);
  }
}

const card = computed(() => {
  if (!url.value) return null;
  try {
    const u = new URL(url.value);
    return {
      hostname: u.hostname,
      pathname: u.pathname === "/" ? "" : u.pathname,
      full: u.toString(),
    };
  } catch {
    return null;
  }
});

async function openUrl() {
  if (!url.value) return;
  try {
    await open(url.value);
  } catch (e) {
    console.error("[link] open failed", e);
  }
}
</script>

<template>
  <div class="link-editor" :style="{ fontSize: (fontSize ?? 14) + 'px' }">
    <textarea
      class="link-editor__input"
      :value="text"
      @input="onInput"
      @blur="onBlur"
      :placeholder="placeholder ?? '粘贴或输入链接…'"
      spellcheck="false"
      autocomplete="off"
    />

    <div v-if="card" class="link-card" @click="openUrl">
      <div class="link-card__domain">{{ card.hostname }}</div>
      <div v-if="card.pathname" class="link-card__path">{{ card.pathname }}</div>
      <div class="link-card__url">{{ card.full }}</div>
      <div class="link-card__hint">点击用默认浏览器打开 ↗</div>
    </div>

    <div v-else class="link-empty">
      <p>还没识别到链接</p>
      <p class="link-empty__hint">支持 http:// / https:// 开头的 URL</p>
    </div>
  </div>
</template>

<style scoped>
.link-editor {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  user-select: none;
}

.link-editor__input {
  flex: 0 0 auto;
  margin: 10px 12px 6px;
  padding: 6px 8px;
  background: rgba(255, 255, 255, 0.5);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 6px;
  font: inherit;
  font-size: 0.95em;
  color: inherit;
  resize: none;
  outline: none;
  min-height: 32px;
  max-height: 80px;
  user-select: text;
}
.link-editor__input:focus {
  background: rgba(255, 255, 255, 0.85);
  border-color: rgba(0, 0, 0, 0.25);
}
.link-editor__input::placeholder {
  opacity: 0.5;
}

.link-card {
  flex: 1;
  margin: 8px 12px 12px;
  padding: 14px 16px;
  background: rgba(255, 255, 255, 0.65);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 10px;
  cursor: pointer;
  transition: background 0.1s, transform 0.05s, border-color 0.1s;
  display: flex;
  flex-direction: column;
  gap: 4px;
  user-select: text;
  overflow: auto;
}
.link-card:hover {
  background: rgba(255, 255, 255, 0.9);
  border-color: rgba(0, 0, 0, 0.2);
}
.link-card:active {
  transform: scale(0.99);
}

.link-card__domain {
  font-size: 1.1em;
  font-weight: 600;
  color: #1d4ed8;
}
.link-card__path {
  font-size: 0.85em;
  opacity: 0.7;
  word-break: break-all;
}
.link-card__url {
  font-size: 0.75em;
  opacity: 0.5;
  word-break: break-all;
  margin-top: 4px;
}
.link-card__hint {
  margin-top: 8px;
  font-size: 0.75em;
  opacity: 0.5;
  text-align: right;
}

.link-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  opacity: 0.5;
  text-align: center;
  padding: 20px;
}
.link-empty p {
  margin: 4px 0;
}
.link-empty__hint {
  font-size: 0.85em;
  opacity: 0.7;
}
</style>
