import React from 'react';
import SudokuTable from './sudoku/SudokuTable';
import { Container, Navbar, Row } from "react-bootstrap";
import logo from './logo.svg'
import './App.css'

function App() {
  return (
    <>
      <header>
        <Navbar bg="dark" variant="dark">
          <Navbar.Brand href="#home">
            <img
              src={logo}
              width="30"
              height="30"
              className="d-inline-block align-top sudoku-logo"
              alt="bla"
            />
      &nbsp;&nbsp;Sudoku
    </Navbar.Brand>
        </Navbar>
      </header>
      <main>
        <Container>
          <Row className="mt-3 justify-content-center">
            <SudokuTable />
          </Row>
        </Container>
      </main>
    </>
  );
}

export default App;                             
