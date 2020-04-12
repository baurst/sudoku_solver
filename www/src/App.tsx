import React from 'react';                      
import SudokuTable from './sudoku/SudokuTable'; 
import Navbar from "react-bootstrap/Navbar";
                                                
function App() {                                
  return (                                      
    <>                       
      <header>
      <Navbar bg="dark" variant="dark">
    <Navbar.Brand href="#home">
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
