import { useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import dayjs from 'dayjs';
import { Todo } from '../types/todo';

function convertToTodos(todos: any[]): Todo[] {
  return todos.map((todo) => ({
    ...todo,
    createdAt: dayjs(todo.createdAt).format("YYYY-MM-DD HH:mm"),
  }));
}

export function useTodos() {
  const [todos, setTodos] = useState<Todo[]>([]);

  const fetchTodos = async () => {
    try {
      const todos = await invoke("get_todos") as any[];
      setTodos(convertToTodos(todos));
    } catch (error) {
      console.error('Failed to fetch todos:', error);
    }
  };

  const addTodo = async (text: string) => {
    try {
      await invoke("post_todo", { text });
      await fetchTodos();
    } catch (error) {
      console.error('Failed to add todo:', error);
    }
  };

  const deleteTodo = async (id: number) => {
    try {
      await invoke("delete_todo", { id });
      await fetchTodos();
    } catch (error) {
      console.error('Failed to delete todo:', error);
    }
  };

  const toggleTodoCompletion = async (id: number, completed: boolean) => {
    try {
      const todo = todos.find(todo => todo.id === id);
      if (todo) {
        await invoke("update_todo", { id, text: todo.text, completed });
        await fetchTodos();
      }
    } catch (error) {
      console.error('Failed to toggle todo completion:', error);
    }
  };

  const sortedTodos = useMemo(() => {
    const sortedByCreatedAt = todos.sort((a, b) => {
      return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
    });
    const sortedByCompleted = sortedByCreatedAt.sort((a, b) => {
      if (a.completed && !b.completed) {
        return 1;
      } else if (!a.completed && b.completed) {
        return -1;
      } else {
        return 0;
      }
    });
    return sortedByCompleted;
  }, [todos]);

  return {
    todos: sortedTodos,
    fetchTodos,
    addTodo,
    deleteTodo,
    toggleTodoCompletion
  };
}
