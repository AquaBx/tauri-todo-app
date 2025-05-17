import { invoke } from '@tauri-apps/api/core';

export interface Todo {
  id: number;
  text: string;
  completed: boolean;
}

export async function loadTodos(): Promise<Todo[]> {
    return await invoke<Todo[]>('load_todos');
}

export async function addTodo(text: string): Promise<Todo | null> {
    return await invoke<Todo>('add_todo', { text });
}

export async function toggleTodo(id: number): Promise<boolean> {
  try {
    await invoke('toggle_todo', { id });
    return true;
  } catch (error) {
    return false;
  }
}

export async function deleteTodo(id: number): Promise<boolean> {
  try {
    await invoke('delete_todo', { id });
    return true;
  } catch (error) {
    return false;
  }
}

export async function saveTodos(todos: Todo[]): Promise<boolean> {
  try {
    await invoke('save_todos', { todos });
    return true;
  } catch (error) {
    return false;
  }
}

export async function getTodos(): Promise<Todo[]> {
  try {
    return await invoke<Todo[]>('get_todos');
  } catch (error) {
    return [];
  }
}