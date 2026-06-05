<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import StickyCard from "../components/StickyCard.vue";
import StickyToolbar from "../components/StickyToolbar.vue";
import TextEditor from "../components/TextEditor.vue";
import TodoEditor from "../components/TodoEditor.vue";
import LinkEditor from "../components/LinkEditor.vue";
import { useSticky } from "../composables/useSticky";

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
    <TextEditor
      v-if="sticky.type === 'text'"
      v-model="content"
      :font-size="sticky.font_size"
    />
    <TodoEditor
      v-else-if="sticky.type === 'todo'"
      :sticky-id="sticky.id"
    />
    <LinkEditor
      v-else-if="sticky.type === 'link'"
      v-model="content"
      :font-size="sticky.font_size"
    />
    <div v-else class="placeholder">
      W4.2 待做：{{ sticky.type }} 便签
    </div>
    <template #toolbar>
      <StickyToolbar :sticky="sticky" />
    </template>
  </StickyCard>
  <div v-else class="loading">加载中…</div>
</template>

<style scoped>
.loading,
.placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  color: #888;
}
</style>
