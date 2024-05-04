import React, { useEffect, useState } from 'react';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import { useTodos } from './hooks/useTodos';
import './App.css';
import TodoItems from './components/TodoItems';
import Box from '@mui/material/Box';


function App() {
  const { todos, fetchTodos, addTodo, deleteTodo, toggleTodoCompletion } = useTodos();
  const [inputTodo, setInputTodo] = useState("");

  useEffect(() => {
    fetchTodos();
  }, []);

  const handleSubmit = async () => {
    await addTodo(inputTodo);
    setInputTodo("");
  };

  return (
      <Box sx={{ height: '100vh', display: 'flex', flexDirection: 'column', justifyContent: 'space-between' }}>
        <Typography variant="h4" sx={{ my: 2, mx: 2, color: 'primary.main', fontWeight: 'bold' }}>Simple ToDo</Typography>
        <Box sx={{ overflowY: 'auto', flexGrow: 1, mx: 2 }}>
          {todos.map(todo => (
            <TodoItems
              key={todo.id}
              todo={todo}
              onCheck={toggleTodoCompletion}
              onDelete={deleteTodo}
            />
          ))}
        </Box>
        <Box sx={{ display: 'flex', mt: 2, mx: 2, mb: 2 }}>
          <TextField
            fullWidth
            value={inputTodo}
            onChange={(e) => setInputTodo(e.target.value)}
            placeholder="Enter a todo..."
            onKeyDown={(e) => {
                if (e.key === 'Enter' && e.keyCode !== 229) {
                handleSubmit();
                }
            }}
          />
          <Button onClick={handleSubmit} disabled={!inputTodo} variant="contained" color="primary" sx={{ ml: 1 }}>POST</Button>
        </Box>
      </Box>
  );
}

export default App;
