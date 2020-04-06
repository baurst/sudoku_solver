import React from 'react';
import './App.css';
import SudokuTable from './sudoku/SudokuTable';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        Sudoku
      <SudokuTable />
      </header>
    </div>
  );
}

export default App;
