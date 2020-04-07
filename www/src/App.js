import React from 'react';

import './App.css';
import Test from './Test'
import SudokuTable from './sudoku/SudokuTable';


class App extends React.PureComponent {
	render() {
		return (
			<div>
				<h1>Hello World!</h1>
				<Test />
				Sudoku
				<SudokuTable ></SudokuTable>
			</div>
		);
	}
}

export default App;