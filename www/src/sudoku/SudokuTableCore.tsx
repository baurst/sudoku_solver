import { useState, useEffect } from "react";
import * as wasm from "sudoku_solver";


interface SudokuTableProps {
  sudoku: number[];
  setValueInSudoku: Function;
  solveSudoku: Function;
  clearSudoku: Function;
}

function useSudokuTableCore(size: number): SudokuTableProps {
  const [sudoku, setSudoku] = useState(new Array(size * size).fill(0));

  const setValueInSudoku = (place: number, value: number) => {
    const currentSudoku = [...sudoku];
    console.log(`set value ${value} in place ${place}.`);
    console.log(currentSudoku.toString());
    currentSudoku[place] = value;
    console.log(currentSudoku.toString());
    setSudoku(currentSudoku);
  };

  const solveSudoku = () => {
    let solvedSudoku = "";
      solvedSudoku = wasm.wasm_solve_sudoku(sudoku.join(""));
    setSudoku(solvedSudoku.split('').map(x=>+x));
  };

  const clearSudoku = () => {
    let emptySudoku = new Array(size * size).fill(0);
    setSudoku(emptySudoku);
  };

  useEffect(() => {
    console.log(sudoku.toString());
  }, [sudoku]);

  return {
    sudoku: sudoku,
    setValueInSudoku: setValueInSudoku,
    solveSudoku: solveSudoku,
    clearSudoku: clearSudoku,
  };
}

export default useSudokuTableCore;
