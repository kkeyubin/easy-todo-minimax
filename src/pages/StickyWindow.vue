<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import StickyCard from "../components/StickyCard.vue";
import TextEditor from "../components/TextEditor.vue";
import { useSticky } from "../composables/useSticky";
import { ipc, type Sticky } from "../ipc";

const route = useRoute();
const id = computed(() => Number(route.params.id));

const { sticky, ready, load } = useSticky(id);
const content = computed({
  get: () => sticky.value?.content ?? "",
  set: (v: string) => {
    if (sticky.value) sticky.value.content = v;
  },
});

onMounted(load);
</script>

<template>
  <StickyCard v-if="ready && sticky" :sticky="sticky">
    <TextEditor v-model="content" :font-size="sticky.font_size" />
  </StickyCard>
  <div v-else class="loading">加载中…</div>
</template>

<style scoped>
.loading {
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  color: #888;
}
</style>
