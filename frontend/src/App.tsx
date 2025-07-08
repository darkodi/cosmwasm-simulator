// src/App.tsx
import React, { useState } from 'react';
import './App.css';
import { SimulationViewer } from './SimulationViewer';
import { SchemaForm } from './SchemaForm';

const App = () => {
  const [selectedAction, setSelectedAction] = useState<'increment' | 'reset'>('increment');

  const handleExecuteSubmit = async (msg: any) => {
    console.log("📤 Executing message:", msg);

    fetch('http://localhost:4000/simulate', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(msg),
    })
      .then((res) => {
        if (!res.ok) throw new Error(`❌ Backend error: ${res.status}`);
        return res.json();
      })
      .then((data) => {
        console.log('✅ Simulation result from backend:', data);
      })
      .catch((err) => {
        console.error('❌ Failed to reach backend:', err);
      });
  };

  const schemaPath = `cw_tpl_osmosis/${selectedAction}_msg.json`;

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
