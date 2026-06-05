<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { convertFileSrc } from "@tauri-apps/api/core";
import { ipc, type Sticky } from "../ipc";

const props = defineProps<{
  sticky: Sticky;
}>();

const appDataDir = ref<string>("");
const imageUrl = ref<string | null>(null);
const loading = ref(false);
const dragging = ref(false);
const errorMsg = ref<string | null>(null);

const hasImage = computed(() => !!props.sticky.image_path);

async function loadAppDataDir() {
  try {
    appDataDir.value = await ipc.appDataDir();
  } catch (e) {
    console.error("[image] get appDataDir failed", e);
  }
}

async function refreshImageUrl() {
  if (props.sticky.image_path && appDataDir.value) {
    const sep = appDataDir.value.includes("\\") ? "\\" : "/";
    const abs = `${appDataDir.value}${sep}${props.sticky.image_path}`;
    imageUrl.value = convertFileSrc(abs);
  } else {
    imageUrl.value = null;
  }
}

onMounted(async () => {
  await loadAppDataDir();
  await refreshImageUrl();
});

const dragUnlisten = ref<(() => void) | null>(null);

onMounted(async () => {
  // Tauri 2 文件拖入
  const win = getCurrentWebviewWindow();
  const unlisten = await win.onDragDropEvent((event) => {
    const t = event.payload.type;
    if (t === "over" || t === "enter") {
      dragging.value = true;
    } else if (t === "leave") {
      dragging.value = false;
    } else if (t === "drop") {
      dragging.value = false;
      const paths = (event.payload as { paths: string[] }).paths;
      handlePaths(paths);
    }
  });
  dragUnlisten.value = unlisten;
});

onUnmounted(() => {
  if (dragUnlisten.value) dragUnlisten.value();
});

watch(
  () => props.sticky.image_path,
  () => refreshImageUrl()
);

async function handlePaths(paths: string[]) {
  if (!paths.length) return;
  const imagePath = paths.find((p) => /\.(png|jpe?g|gif|webp|bmp)$/i.test(p));
  if (!imagePath) {
    errorMsg.value = "请拖入图片文件（png/jpg/gif/webp/bmp）";
    setTimeout(() => (errorMsg.value = null), 3000);
    return;
  }
  await doAdd(imagePath);
}

async function doAdd(srcPath: string) {
  loading.value = true;
  errorMsg.value = null;
  try {
    await ipc.addStickyImage(props.sticky.id, srcPath);
    // image_path 变化 → watcher 触发 refreshImageUrl
  } catch (e: any) {
    console.error("[image] add failed", e);
    errorMsg.value = `添加失败：${e?.message ?? e}`;
  } finally {
    loading.value = false;
  }
}

async function pickFile() {
  try {
    const path = await openDialog({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "图片",
          extensions: ["png", "jpg", "jpeg", "gif", "webp", "bmp"],
        },
      ],
    });
    if (typeof path === "string") {
      await doAdd(path);
    }
  } catch (e) {
    console.error("[image] open dialog failed", e);
  }
}

async function onPaste(e: ClipboardEvent) {
  const items = e.clipboardData?.items;
  if (!items) return;
  for (const item of items) {
    if (item.type.startsWith("image/")) {
      const file = item.getAsFile();
      if (!file) continue;
      e.preventDefault();
      // 转 RGBA → Uint8Array → Array<number>
      const img = new Image();
      img.onload = async () => {
        const canvas = document.createElement("canvas");
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;
        const ctx = canvas.getContext("2d");
        if (!ctx) return;
        ctx.drawImage(img, 0, 0);
        const data = ctx.getImageData(0, 0, canvas.width, canvas.height);
        loading.value = true;
        try {
          // 简化：暂时让用户用拖入 / 选文件
          errorMsg.value = "粘贴暂未启用，请用拖入或选文件";
        } finally {
          loading.value = false;
        }
      };
      img.src = URL.createObjectURL(file);
      return;
    }
  }
}

onMounted(() => {
  document.addEventListener("paste", onPaste);
});
onUnmounted(() => {
  document.removeEventListener("paste", onPaste);
});

async function removeImage() {
  if (!confirm("确定删除这张图片？")) return;
  loading.value = true;
  try {
    await ipc.removeStickyImage(props.sticky.id);
    imageUrl.value = null;
  } catch (e) {
    console.error("[image] remove failed", e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div
    class="image-editor"
    :class="{ 'image-editor--dragging': dragging, 'image-editor--loading': loading }"
  >
    <div v-if="hasImage && imageUrl" class="image-editor__view">
      <img class="image-editor__img" :src="imageUrl" alt="便签图片" />
      <button
        class="image-editor__del"
        @click="removeImage"
        title="删除图片"
      >×</button>
    </div>

    <div v-else class="image-editor__empty">
      <p class="image-editor__hint">
        {{
          dragging
            ? "松手添加图片"
            : loading
            ? "处理中…"
            : "拖入图片 / 点 + 选文件"
        }}
      </p>
      <button class="image-editor__add" @click="pickFile" :disabled="loading">
        + 选图片
      </button>
    </div>

    <div v-if="errorMsg" class="image-editor__error">{{ errorMsg }}</div>
  </div>
</template>

<style scoped>
.image-editor {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 0;
  overflow: hidden;
  transition: background 0.15s;
}
.image-editor--dragging {
  background: rgba(0, 100, 200, 0.15);
  outline: 2px dashed rgba(0, 100, 200, 0.5);
  outline-offset: -8px;
}
.image-editor--loading {
  opacity: 0.7;
}

.image-editor__view {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.image-editor__img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  user-select: none;
  -webkit-user-drag: none;
}
.image-editor__del {
  position: absolute;
  top: 6px;
  right: 6px;
  width: 24px;
  height: 24px;
  background: rgba(0, 0, 0, 0.5);
  color: #fff;
  border: 0;
  border-radius: 50%;
  font-size: 16px;
  line-height: 1;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.1s, background 0.1s;
}
.image-editor__view:hover .image-editor__del {
  opacity: 1;
}
.image-editor__del:hover {
  background: #dc2626;
}

.image-editor__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: inherit;
  opacity: 0.6;
}
.image-editor__hint {
  margin: 0;
  font-size: 13px;
}
.image-editor__add {
  padding: 6px 14px;
  background: rgba(0, 0, 0, 0.1);
  border: 1px dashed currentColor;
  border-radius: 6px;
  font: inherit;
  color: inherit;
  cursor: pointer;
  opacity: 1;
  transition: background 0.1s;
}
.image-editor__add:hover:not(:disabled) {
  background: rgba(0, 0, 0, 0.18);
}
.image-editor__add:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.image-editor__error {
  position: absolute;
  bottom: 8px;
  left: 8px;
  right: 8px;
  padding: 6px 10px;
  background: rgba(220, 38, 38, 0.9);
  color: #fff;
  font-size: 11px;
  border-radius: 4px;
  text-align: center;
}
</style>
