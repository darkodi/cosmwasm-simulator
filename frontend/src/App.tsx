import { useEffect, useState } from 'react';
import './App.css';

function App() {
  const [simulation, setSimulation] = useState<any>(null);

  useEffect(() => {
    fetch('/simulations/latest_counter_increment.json')
      .then((res) => res.json())
      .then(setSimulation)
      .catch(console.error);
  }, []);

  if (!simulation) return <div>Loading simulation result...</div>;

  return (
    <div style={{ padding: '2rem', fontFamily: 'monospace' }}>
      <h1>🔍 Simulation Result</h1>
      <pre>{JSON.stringify(simulation, null, 2)}</pre>
    </div>
  );
}

export default App;
