import React from 'react';
import Typography from '@mui/material/Typography';
import Card from '@mui/material/Card';
import Checkbox from '@mui/material/Checkbox';
import DeleteForeverIcon from '@mui/icons-material/DeleteForever';
import { Todo } from '../types/todo';

function TodoItems({ todo, onCheck, onDelete }: { todo: Todo; onCheck: (id: number, completed: boolean) => void; onDelete: (id: number) => void }) {
  return (
    <Card sx={{ display: 'flex', justifyContent: 'space-between', p: 1, mt: 1 }}>
      <Checkbox
        checked={todo.completed}
        onChange={(e) => onCheck(todo.id, e.target.checked)}
      />
      <Typography sx={{ flexGrow: 1, mx: 1, lineHeight: '42px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', textDecoration: todo.completed ? 'line-through' : 'none' }}>
        {todo.text}
      </Typography>
      <Typography sx={{ mx: 1, alignSelf: 'center', color: 'text.secondary', fontSize: '0.75rem', whiteSpace: 'nowrap' }}>
        {todo.createdAt}
      </Typography>
      <DeleteForeverIcon
        onClick={() => onDelete(todo.id)}
        sx={{ cursor: 'pointer', alignSelf: 'center' }}
      />
    </Card>
  );
}

export default TodoItems;
