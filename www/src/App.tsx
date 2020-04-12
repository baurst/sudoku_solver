import React from 'react';                      
import SudokuTable from './sudoku/SudokuTable'; 
import Navbar from "react-bootstrap/Navbar";
import logo from './logo.svg'
                                                
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
        className="d-inline-block align-top"
        alt="bla"
      />{' '}
      Sudoku
    </Navbar.Brand>
  </Navbar>           
      </header>  
      <main>
      <SudokuTable />                           
        </main>                               
    </>                                      
  );                                            
}                                               
                                                
export default App;                             
