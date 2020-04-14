import React, { useState } from "react";
import useSudokuTableCore from "./SudokuTableCore";
import { Button, Container, Row, Modal } from "react-bootstrap";
import './SudokuTable.css'

function tableRow(
  size: number,
  rowNr: number,
  sudoku: number[],
  setValueInSudoku: Function,
  isUserInput: boolean[],

) {
  const sizeElements = Array(size)
    .fill(0)
    .map((v, i) => rowNr * 9 + i);

  return (
    <>
      <tr>
        {sizeElements.map((entry) =>
          tableEntry(entry, sudoku, setValueInSudoku, isUserInput)
        )}
      </tr>
    </>
  );
}

const styles = {
  text_normal: {
    fontWeight: "normal",
  } as React.CSSProperties,
  text_bold: {
    fontWeight: "bold",
  } as React.CSSProperties,
}

function tableEntry(
  entryNr: number,
  sudoku: number[],
  setValueInSudoku: Function,
  isUserInput: boolean[],
) {
  const entryStr = `${entryNr}`;
  const cellNames = [
    "left-top",
    "middle-top",
    "right-top",
    "left-middle",
    "middle-middle",
    "right-middle",
    "left-bottom",
    "middle-bottom",
    "right-bottom"];

  const row = Math.floor(entryNr / 9);
  const col = entryNr % 9;
  const meta_cell_row = Math.floor(row / 3);
  const meta_cell_col = Math.floor(col / 3);
  const pos_row = row - meta_cell_row * 3;
  const pos_col = col - meta_cell_col * 3;
  const pos_idx = pos_row * 3 + pos_col;
  const styleName = cellNames[pos_idx];

  return (
    <>
      <td className={styleName}>
        <input
          value={sudoku[entryNr] === 0 ? "" : sudoku[entryNr]}
          name={entryStr}
          type="text"
          pattern="[0-9]*"
          style={isUserInput[entryNr] ? styles.text_bold : styles.text_normal}
          maxLength={1}
          size={1}
          onChange={(event: React.FormEvent<HTMLInputElement>) => {
            console.log(
              `change ${event.currentTarget.name} to ${event.currentTarget.value}`
            );
            if (!isNaN(parseInt(event.currentTarget.value)) || event.currentTarget.value === "") {
              let parseValue = parseInt(event.currentTarget.value);
              if (isNaN(parseValue)) {
                setValueInSudoku(entryNr, 0);
              }
              else {
                setValueInSudoku(entryNr, parseInt(event.currentTarget.value));
              }
            }
          }}
        />
      </td>
    </>
  );
}

const SudokuTable: React.FC = () => {
  const size = 9;
  const sudokuTableCore = useSudokuTableCore(size);
  const sizeElements = Array(size)
    .fill(0)
    .map((v, i) => i);
  const [isSolving, setSolving] = useState(false);

  const [showWarning, setShowWarning] = useState(false);

  const ConflictWarning: React.FC = () => {
    const handleClose = () => setShowWarning(false);

    return (
      <Modal show={showWarning} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>WARNING</Modal.Title>
        </Modal.Header>
        <Modal.Body><p> Sudoku contains unsolvable conflict(s)!</p> <p> Please fix the conflicts and try again!</p> </Modal.Body>
        <Modal.Footer>
          <Button variant="primary" onClick={handleClose}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    );
  }

  const handleClick = () => {
    if (!isSolving) {
      let sudokuIsOkay = sudokuTableCore.checkSudokuIsSolvable();
      if(sudokuIsOkay){
        setSolving(true);
        sudokuTableCore.solveSudoku();
        setSolving(false);
      } else {
        setShowWarning(true);
      }

    }
  };

  const handleClearClick = () => {
    sudokuTableCore.clearSudoku();
  };

  return (
    <Container>
      <ConflictWarning> </ConflictWarning>
      <Row className="justify-content-center">
        <table className="sudoku-table">
          <tbody>
            {sizeElements.map((row) =>
              tableRow(
                size,
                row,
                sudokuTableCore.sudoku,
                sudokuTableCore.setValueInSudoku,
                sudokuTableCore.isUserInput
              )
            )}
          </tbody>
        </table>
      </Row>
      <Row className="mt-3 justify-content-center">
        <Button className="mr-2" variant="primary" disabled={isSolving} onClick={handleClick}>
          {isSolving ? "Solving…" : "Solve"}
        </Button>
        <Button className="ml-2" variant="primary" disabled={isSolving} onClick={handleClearClick}>
          Clear
      </Button>
      </Row>
    </Container>
  );
};

export default SudokuTable;
