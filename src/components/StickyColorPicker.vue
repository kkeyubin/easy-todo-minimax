<script setup lang="ts">
import { STICKY_COLORS } from "../types/sticky";

defineProps<{
  modelValue: string;
}>();

defineEmits<{
  "update:modelValue": [id: string];
  "close": [];
}>();
</script>

<template>
  <div class="color-picker" @mousedown.stop @click.stop>
    <button
      v-for="c in STICKY_COLORS"
      :key="c.id"
      class="color-picker__swatch"
      :class="{ 'color-picker__swatch--active': modelValue === c.id }"
      :style="{ background: c.bg, borderColor: c.border }"
      :title="c.id"
      @click="$emit('update:modelValue', c.id); $emit('close')"
    />
  </div>
</template>

<style scoped>
.color-picker {
  display: flex;
  gap: 4px;
  padding: 6px 8px;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  backdrop-filter: blur(8px);
}
.color-picker__swatch {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 1.5px solid;
  cursor: pointer;
  padding: 0;
  outline: none;
  transition: transform 0.1s;
}
.color-picker__swatch:hover {
  transform: scale(1.15);
}
.color-picker__swatch--active {
  box-shadow: 0 0 0 2px #fff, 0 0 0 3.5px #2563eb;
}
</style>
