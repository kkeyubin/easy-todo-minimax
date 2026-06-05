<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { ipc, type Todo } from "../ipc";

const props = defineProps<{
  stickyId: number;
}>();

const todos = ref<Todo[]>([]);
const newText = ref("");
const doneCollapsed = ref(true);
const editingId = ref<number | null>(null);
const editingText = ref("");

const active = computed(() => todos.value.filter((t) => !t.done));
const done = computed(() => todos.value.filter((t) => t.done));

async function load() {
  try {
    todos.value = await ipc.listTodos(props.stickyId);
  } catch (e) {
    console.error("[todo] load failed", e);
  }
}

onMounted(load);

async function addTodo() {
  const text = newText.value.trim();
  if (!text) return;
  try {
    const t = await ipc.addTodo(props.stickyId, text);
    todos.value.push(t);
    newText.value = "";
  } catch (e) {
    console.error("[todo] add failed", e);
  }
}

async function toggle(t: Todo) {
  const next = t.done ? 0 : 1;
  t.done = next;
  try {
    await ipc.updateTodo(t.id, { done: next });
  } catch (e) {
    console.error("[todo] update done failed", e);
  }
}

async function remove(t: Todo) {
  try {
    await ipc.deleteTodo(t.id);
    todos.value = todos.value.filter((x) => x.id !== t.id);
  } catch (e) {
    console.error("[todo] delete failed", e);
  }
}

function startEdit(t: Todo) {
  editingId.value = t.id;
  editingText.value = t.text;
}

async function commitEdit(t: Todo) {
  const text = editingText.value.trim();
  editingId.value = null;
  if (!text || text === t.text) return;
  t.text = text;
  try {
    await ipc.updateTodo(t.id, { text });
  } catch (e) {
    console.error("[todo] update text failed", e);
  }
}
</script>

<template>
  <div class="todo-editor">
    <ul v-if="active.length" class="todo-list">
      <li v-for="t in active" :key="t.id" class="todo-item">
        <label class="todo-item__check">
          <input
            type="checkbox"
            :checked="t.done === 1"
            @change="toggle(t)"
          />
          <span class="todo-item__checkmark"></span>
        </label>
        <span
          v-if="editingId !== t.id"
          class="todo-item__text"
          @dblclick="startEdit(t)"
        >{{ t.text }}</span>
        <input
          v-else
          class="todo-item__edit"
          v-model="editingText"
          @blur="commitEdit(t)"
          @keydown.enter="commitEdit(t)"
          @keydown.esc="editingId = null"
          spellcheck="false"
          v-focus
        />
        <button
          class="todo-item__del"
          @click="remove(t)"
          title="删除"
        >×</button>
      </li>
    </ul>

    <!-- 已完成折叠区 -->
    <div v-if="done.length" class="todo-done">
      <button
        class="todo-done__toggle"
        @click="doneCollapsed = !doneCollapsed"
      >
        <span class="todo-done__arrow">{{ doneCollapsed ? "▸" : "▾" }}</span>
        已完成 {{ done.length }}
      </button>
      <ul v-if="!doneCollapsed" class="todo-list todo-list--done">
        <li
          v-for="t in done"
          :key="t.id"
          class="todo-item todo-item--done"
        >
          <label class="todo-item__check">
            <input
              type="checkbox"
              :checked="t.done === 1"
              @change="toggle(t)"
            />
            <span class="todo-item__checkmark"></span>
          </label>
          <span class="todo-item__text">{{ t.text }}</span>
          <button
            class="todo-item__del"
            @click="remove(t)"
            title="删除"
          >×</button>
        </li>
      </ul>
    </div>

    <!-- 新增输入框 -->
    <div class="todo-add">
      <input
        v-model="newText"
        @keydown.enter.prevent="addTodo"
        placeholder="+ 新待办（Enter 添加）"
        spellcheck="false"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { type Directive } from "vue";

const vFocus: Directive<HTMLInputElement> = {
  mounted: (el) => el.focus(),
};

export default {
  directives: { focus: vFocus },
};
</script>

<style scoped>
.todo-editor {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  font-size: 14px;
  line-height: 1.5;
  user-select: none;
  overflow: hidden;
}

.todo-list {
  list-style: none;
  margin: 0;
  padding: 8px 12px 0;
  flex: 1;
  overflow-y: auto;
}
.todo-list--done {
  padding: 4px 0 0 26px;
  flex: 0 0 auto;
}

.todo-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
  color: inherit;
}
.todo-item--done .todo-item__text {
  text-decoration: line-through;
  opacity: 0.5;
}

.todo-item__check {
  position: relative;
  flex: 0 0 16px;
  width: 16px;
  height: 16px;
  cursor: pointer;
}
.todo-item__check input {
  position: absolute;
  opacity: 0;
  width: 100%;
  height: 100%;
  margin: 0;
  cursor: pointer;
}
.todo-item__checkmark {
  position: absolute;
  inset: 0;
  border: 1.5px solid currentColor;
  border-radius: 50%;
  opacity: 0.4;
  transition: opacity 0.1s, background 0.1s;
}
.todo-item__check input:checked + .todo-item__checkmark {
  background: currentColor;
  opacity: 0.6;
}
.todo-item__check input:checked + .todo-item__checkmark::after {
  content: "✓";
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 11px;
  font-weight: bold;
}

.todo-item__text {
  flex: 1;
  user-select: text;
  cursor: text;
  word-break: break-word;
}

.todo-item__edit {
  flex: 1;
  background: transparent;
  border: 0;
  border-bottom: 1px solid currentColor;
  outline: 0;
  color: inherit;
  font: inherit;
  padding: 0;
  user-select: text;
}

.todo-item__del {
  flex: 0 0 18px;
  width: 18px;
  height: 18px;
  border: 0;
  background: transparent;
  color: inherit;
  opacity: 0;
  font-size: 16px;
  line-height: 1;
  cursor: pointer;
  border-radius: 4px;
  transition: opacity 0.1s, background 0.1s;
}
.todo-item:hover .todo-item__del {
  opacity: 0.4;
}
.todo-item__del:hover {
  opacity: 1 !important;
  background: rgba(220, 38, 38, 0.15);
  color: #dc2626;
}

.todo-done {
  flex: 0 0 auto;
  border-top: 1px dashed rgba(0, 0, 0, 0.08);
  margin: 4px 12px 0;
  padding: 4px 0;
}
.todo-done__toggle {
  display: flex;
  align-items: center;
  gap: 4px;
  background: transparent;
  border: 0;
  font: inherit;
  font-size: 11px;
  color: inherit;
  opacity: 0.6;
  cursor: pointer;
  padding: 0;
}
.todo-done__toggle:hover {
  opacity: 1;
}
.todo-done__arrow {
  font-size: 10px;
  display: inline-block;
  width: 10px;
  text-align: center;
}

.todo-add {
  flex: 0 0 auto;
  padding: 6px 12px 10px;
}
.todo-add input {
  width: 100%;
  background: transparent;
  border: 0;
  border-bottom: 1px dashed currentColor;
  outline: 0;
  color: inherit;
  font: inherit;
  font-size: 12px;
  padding: 4px 0;
  opacity: 0.6;
  transition: opacity 0.1s, border-color 0.1s;
  user-select: text;
}
.todo-add input:hover,
.todo-add input:focus {
  opacity: 1;
  border-bottom-color: currentColor;
  border-bottom-style: solid;
}
.todo-add input::placeholder {
  color: inherit;
  opacity: 0.5;
}
</style>
