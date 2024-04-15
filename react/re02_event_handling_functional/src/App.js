import './App.css';
import { useState } from 'react';

const Toggle = () => {
	const [curToggle, setCurToggle] = useState(true);

	const handleClick = () => {
		setCurToggle(!curToggle);
	};

	return(
		<button onClick={handleClick}>
			{curToggle ? 'ON' : 'OFF'}
		</button>
	);
};

function App() {
	return (
		<div className="App">
			<header className="App-header">
				<Toggle />
			</header>
		</div>
	);
}

export default App;
