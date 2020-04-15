import { useState, useEffect } from "react";
import * as wasm from "sudoku_solver";


interface SudokuTableProps {
  sudoku: number[];
  isUserInput: boolean[];
  setValueInSudoku: Function;
  checkSudokuIsSolvable: Function;
  solveSudoku: Function;
  insertSampleSudoku: Function;
  clearSudoku: Function;
}

function useSudokuTableCore(size: number): SudokuTableProps {
  const [sudoku, setSudoku] = useState(new Array(size * size).fill(0));
  const [isUserInput, setIsUserInput] = useState(new Array(size * size).fill(false));

  const setValueInSudoku = (place: number, value: number) => {
    const currentSudoku = [...sudoku];
    console.log(`set value ${value} in place ${place}.`);
    console.log(currentSudoku.toString());
    currentSudoku[place] = value;
    isUserInput[place] = true;
    console.log(currentSudoku.toString());
    setSudoku(currentSudoku);

  };

  const insertSampleSudoku = () => {
    let example_sudoku = "";
    example_sudoku = wasm.wasm_get_sample_sudoku_string(Math.random());
    let isUserInput = new Array(size * size).fill(false);
    let example_sudoku_array = example_sudoku.split('').map(x=>+x)
    for (let index = 0; index < example_sudoku.length; ++index) {
      isUserInput[index] = example_sudoku_array[index] !== 0;
    }
    setIsUserInput(isUserInput);
    setSudoku(example_sudoku_array);
  };

  const solveSudoku = () => {
    let solvedSudoku = "";
    solvedSudoku = wasm.wasm_solve_sudoku(sudoku.join(""));
    setSudoku(solvedSudoku.split('').map(x=>+x));
  };

  const checkSudokuIsSolvable = () => {
    let sudokuHasConflict = wasm.wasm_sudoku_contains_conflicts(sudoku.join(""));
    return !sudokuHasConflict;
  };

  const clearSudoku = () => {
    let emptySudoku = new Array(size * size).fill(0);
    let emptyIsUserInput = new Array(size * size).fill(false);
    setSudoku(emptySudoku);
    setIsUserInput(emptyIsUserInput);
  };

  useEffect(() => {
    console.log(sudoku.toString());
  }, [sudoku]);

  return {
    sudoku: sudoku,
    isUserInput: isUserInput,
    setValueInSudoku: setValueInSudoku,
    checkSudokuIsSolvable: checkSudokuIsSolvable,
    insertSampleSudoku: insertSampleSudoku,
    solveSudoku: solveSudoku,
    clearSudoku: clearSudoku,
  };
}

export default useSudokuTableCore;
