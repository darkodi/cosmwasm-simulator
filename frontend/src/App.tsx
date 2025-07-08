import React, { useState } from 'react';
import './App.css';
import { SimulationViewer } from './SimulationViewer';
import { SchemaForm } from './SchemaForm';
import { useContracts } from './hooks/useContracts';

const App = () => {
  const { contracts, loading } = useContracts();

  const contractNames = Object.keys(contracts);
  const [selectedContract, setSelectedContract] = useState<string>(contractNames[0] || '');
  const [selectedAction, setSelectedAction] = useState<string>('');

  const handleContractChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newContract = e.target.value;
    setSelectedContract(newContract);
    const actions = contracts[newContract];
    setSelectedAction(actions?.[0] || '');
  };

  const handleExecuteSubmit = async (msg: any) => {
    console.log("📤 Executing message:", msg);

    fetch('http://localhost:4000/simulate', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(msg),
    })
      .then(res => res.json())
      .then(data => console.log('✅ Simulation result from backend:', data))
      .catch(err => console.error('❌ Failed to reach backend:', err));
  };

  const schemaPath = selectedContract && selectedAction
    ? `${selectedContract}/${selectedAction}_msg.json`
    : '';

  if (loading) return <p>Loading available contracts...</p>;

  return (
    <div className="App">
      <h1>🧪 CosmWasm Simulation Dashboard</h1>

      <label>
        Select contract:&nbsp;
        <select value={selectedContract} onChange={handleContractChange}>
          {contractNames.map((contract) => (
            <option key={contract} value={contract}>{contract}</option>
          ))}
        </select>
      </label>

      <br />

      <label>
        Select action:&nbsp;
        <select
          value={selectedAction}
          onChange={(e) => setSelectedAction(e.target.value)}
          disabled={!selectedContract}
        >
          {(contracts[selectedContract] || []).map((action) => (
            <option key={action} value={action}>{action}</option>
          ))}
        </select>
      </label>

      <h2>📤 Execute Message</h2>
      {schemaPath && (
        <SchemaForm schemaPath={schemaPath} onSubmit={handleExecuteSubmit} />
      )}

      <h2>🔁 Simulation Output</h2>
      <SimulationViewer />
    </div>
  );
};

export default App;
