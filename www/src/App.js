import React from 'react';

import './App.css';
import SudokuTable from './sudoku/SudokuTable';


class App extends React.PureComponent {
	render() {
		return (
			<div>
				Sudoku
				<SudokuTable ></SudokuTable>
			</div>
		);
	}
}

export default App;