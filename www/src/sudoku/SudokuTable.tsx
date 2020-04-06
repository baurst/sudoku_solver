import React, { useState } from "react";
import useSudokuTableCore from "./SudokuTableCore";
import Button from "react-bootstrap/Button";
import * as wasm from "sudoku_solver";

function tableRow(
  size: number,
  rowNr: number,
  sudoku: number[],
  setValueInSudoku: Function
) {
  const sizeElements = Array(size)
    .fill(0)
    .map((v, i) => rowNr * 9 + i);

  return (
    <>
      <tr>
        {sizeElements.map((entry) =>
          tableEntry(entry, sudoku, setValueInSudoku)
        )}
      </tr>
    </>
  );
}

function tableEntry(
  entryNr: number,
  sudoku: number[],
  setValueInSudoku: Function
) {
  const entryStr = `${entryNr}`;

  return (
    <>
      <td>
        <input
          value={sudoku[entryNr] === 0 ? "" : sudoku[entryNr]}
          name={entryStr}
          type="text"
          maxLength={1}
          size={1}
          onChange={(event: React.FormEvent<HTMLInputElement>) => {
            console.log(
              `change ${event.currentTarget.name} to ${event.currentTarget.value}`
            );
            setValueInSudoku(entryNr, parseInt(event.currentTarget.value));
          }}
        />
      </td>
    </>
  );
}

const SudokuTable: React.FC = () => {
	
	var a = wasm.wasm_solve_sudoku("006037508700010900130050020002908000050020430600000090200005704003100060498600000");

	console.log(a);

  const size = 9;
  const sudokuTableCore = useSudokuTableCore(size);
  const sizeElements = Array(size)
    .fill(0)
    .map((v, i) => i);
  const [isSolving, setSolving] = useState(false);

  const handleClick = () => {
    if (!isSolving) {
      setSolving(true);
      sudokuTableCore.solveSudoku().then(() => setSolving(false));
    }
  };

  return (
    <>
      <table>
        <tbody>
          {sizeElements.map((row) =>
            tableRow(
              size,
              row,
              sudokuTableCore.sudoku,
              sudokuTableCore.setValueInSudoku
            )
          )}
        </tbody>
      </table>
      Sudoku String = {sudokuTableCore.sudoku}
      <Button variant="primary" disabled={isSolving} onClick={handleClick}>
        {isSolving ? "Solving…" : "Solve"}
      </Button>
    </>
  );
};

export default SudokuTable;
