import React, { useState } from 'react';

import TodoList from './components/TodoList';
import AddTodo from './components/AddTodo';
import './styles/App.css';

const App: React.FC = () => {

  const [todos, setTodos] = useState<Todo[]>([]);

  const addTodo = (text: string) => {

     setTodos([...todos, { id: todos.length + 1, text, completed: false }]);

};

const toggleTodo = (id: number) => {

  setTodos (

    todos.map((todo) =>

    todo.id === id ? { ...todo, completed: !todo.completed } : todo

  ));
};

const deleteTodo = (id: number) => {

   setTodos(todos.filter((todo) => todo.id !== id));

};

return (

<div className="container">

<h1>Tauri Todo App</h1>

<TodoList todos={todos} toggleTodo={toggleTodo} deleteTodo={deleteTodo} />

<AddTodo addTodo={addTodo} />

</div>

);
};

export default App;