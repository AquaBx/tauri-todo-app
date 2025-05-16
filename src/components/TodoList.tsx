import React from 'react';

interface TodoListProps {
  todos: Todo[];
  toggleTodo: (id: number) => void;
  deleteTodo: (id: number) => void;
}

const TodoList: React.FC<TodoListProps> = ({ todos, toggleTodo, deleteTodo }) => (
  <ul className="todo-list">
    {todos.map(({ id, text, completed }) => (
      <li key={id} className={`todo-item ${completed ? 'completed' : ''}`}>
        <label className="checkbox-container">
          <input
            type="checkbox"
            checked={completed}
            onChange={() => toggleTodo(id)}
            aria-label={`Mark "${text}" as completed`}
          />
          <span className="checkmark"></span>
          <span className="todo-text">{text}</span>
        </label>
        <button
          className="btn btn-danger"
          onClick={() => deleteTodo(id)}
          aria-label={`Delete "${text}"`}
        >
          &times;
        </button>
      </li>
    ))}
  </ul>
);

export default TodoList;
