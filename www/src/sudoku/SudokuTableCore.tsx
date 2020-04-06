import { useState, useEffect } from "react";

interface SudokuTableProps {
  sudoku: number[];
  setValueInSudoku: Function;
  solveSudoku: Function;
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
    //TODO
    let solvedSudoku = new Array(81).fill(0);
    return new Promise((resolve) => {
      solvedSudoku = new Array(81).fill(1);
      setTimeout(resolve, 1);
    }).then(() => setSudoku(solvedSudoku));
  };

  useEffect(() => {
    console.log(sudoku.toString());
  }, [sudoku]);

  return {
    sudoku: sudoku,
    setValueInSudoku: setValueInSudoku,
    solveSudoku: solveSudoku,
  };
}

export default useSudokuTableCore;
