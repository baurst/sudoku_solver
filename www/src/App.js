import React from 'react';

import './App.css';
import Test from './Test'

class App extends React.PureComponent {
	render() {
		return (
			<div>
				<h1>Hello World!</h1>
				<Test />
			</div>
		);
	}
}

export default App;