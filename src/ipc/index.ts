import { invoke } from "@tauri-apps/api/core";

export interface Sticky {
  id: number;
  type: StickyType;
  title: string;
  color: string;
  x: number;
  y: number;
  width: number;
  height: number;
  font_size: number;
  pinned: number;
  z_order: number;
  content: string;
  image_path: string | null;
  link_url: string | null;
  created_at: string;
  updated_at: string;
}

export type StickyType = "text" | "todo" | "link" | "image";

export interface StickyPatch {
  type?: StickyType;
  title?: string;
  color?: string;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  font_size?: number;
  pinned?: number;
  z_order?: number;
  content?: string;
  image_path?: string | null;
  link_url?: string | null;
}

export interface Todo {
  id: number;
  sticky_id: number;
  text: string;
  done: number;
  sort_order: number;
  created_at: string;
}

export const ipc = {
  list: () => invoke<Sticky[]>("list_stickies"),
  get: (id: number) => invoke<Sticky>("get_sticky", { id }),
  create: (stickyType: StickyType, x: number, y: number) =>
    invoke<Sticky>("create_sticky", { stickyType, x, y }),
  update: (id: number, patch: StickyPatch) =>
    invoke<void>("update_sticky", { id, patch }),
  delete: (id: number) => invoke<void>("delete_sticky", { id }),

  listTodos: (stickyId: number) => invoke<Todo[]>("list_todos", { stickyId }),
  addTodo: (stickyId: number, text: string) =>
    invoke<Todo>("add_todo", { stickyId, text }),
  updateTodo: (
    id: number,
    patch: { text?: string; done?: number; sort_order?: number }
  ) => invoke<void>("update_todo", { id, patch }),
  deleteTodo: (id: number) => invoke<void>("delete_todo", { id }),
};
