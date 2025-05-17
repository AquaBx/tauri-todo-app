import React, { useState, useEffect } from 'react';
import TodoList from './components/TodoList';
import AddTodo from './components/AddTodo';
import { loadTodos, addTodo, toggleTodo, deleteTodo, Todo } from './api';
import './styles/App.css';

const App: React.FC = () => {
  const [todos, setTodos] = useState<Todo[]>([]);
  
  useEffect(() => {
    const fetchTodos = async () => {
      const todosData = await loadTodos();
      setTodos(todosData);
    };
    fetchTodos();
  }, []);
  
  const handleAddTodo = async (text: string) => {
    const newTodo = await addTodo(text);
    if (newTodo) {
      setTodos(prevTodos => [...prevTodos, newTodo]);
    }
  };
  
  const handleToggleTodo = async (id: number) => {
    const success = await toggleTodo(id);
    if (success) {
      setTodos(prevTodos =>
        prevTodos.map(todo =>
          todo.id === id ? { ...todo, completed: !todo.completed } : todo
        )
      );
    }
  };
  
  const handleDeleteTodo = async (id: number) => {
    const success = await deleteTodo(id);
    if (success) {
      setTodos(prevTodos => 
        prevTodos.filter(todo => todo.id !== id)
      );
    }
  };
  
  return (
    <div className="container">
      <h1>Tauri Todo App</h1>
      <TodoList 
        todos={todos} 
        toggleTodo={handleToggleTodo} 
        deleteTodo={handleDeleteTodo} 
      />
      <AddTodo addTodo={handleAddTodo} />
    </div>
  );
};

export default App;