import React, { useState } from 'react';
import './App.css';
import { SimulationViewer } from './SimulationViewer';
import { SchemaForm } from './SchemaForm';

const App = () => {
  const [selectedAction, setSelectedAction] = useState<'increment' | 'reset'>('increment');

  const handleExecuteSubmit = (msg: any) => {
    console.log("📤 Executing message:", msg);
    // this will later trigger the backend simulation
  };

  const schemaPath = selectedAction === 'increment' ? 'increment_msg.json' : 'reset_msg.json';

  return (
    <div className="App">
      <h1>🧪 CosmWasm Simulation Dashboard</h1>

      <h2>📤 Execute Message</h2>
      <label>
        Select action:&nbsp;
        <select value={selectedAction} onChange={(e) => setSelectedAction(e.target.value as any)}>
          <option value="increment">increment</option>
          <option value="reset">reset</option>
        </select>
      </label>

      <SchemaForm schemaPath={schemaPath} onSubmit={handleExecuteSubmit} />

      <h2>🔁 Simulation Output</h2>
      <SimulationViewer />
    </div>
  );
};

export default App;
