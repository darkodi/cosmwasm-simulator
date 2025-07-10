import React, { useState, useEffect } from 'react';
import './App.css';
import { SimulationViewer } from './SimulationViewer';
import { SchemaForm } from './SchemaForm';
import { useContracts } from './hooks/useContracts';

const App = () => {
  const { contracts, loading } = useContracts();

  const contractNames = Object.keys(contracts);
  const [selectedContract, setSelectedContract] = useState<string>('');
  const [selectedAction, setSelectedAction] = useState<string>('');
  const [lastSimulationTime, setLastSimulationTime] = useState<number>(0);

  // Fork state
  const [forking, setForking] = useState(false);
  const [forkStatus, setForkStatus] = useState('');

  useEffect(() => {
    if (
      selectedContract === '' &&
      selectedAction === '' &&
      Object.keys(contracts).length > 0
    ) {
      const firstContract = Object.keys(contracts)[0];
      const firstAction = contracts[firstContract]?.[0] || '';

      console.log('🚀 Initializing contract/action:', firstContract, firstAction);
      setSelectedContract(firstContract);
      setSelectedAction(firstAction);
    }
  }, [contracts]);

  const handleContractChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newContract = e.target.value;
    const newAction = contracts[newContract]?.[0] || '';
    setSelectedContract(newContract);
    setSelectedAction(newAction);
    console.log('🔁 Contract changed to:', newContract, ' → Action:', newAction);
  };

  const handleActionChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newAction = e.target.value;
    setSelectedAction(newAction);
    console.log('🔁 Action changed to:', newAction);
  };

  const handleExecuteSubmit = async (msg: any) => {
    const payload = {
      contract: selectedContract,
      action: selectedAction,
      msg,
    };

    console.log('📤 Sending simulation payload:', payload);

    try {
      const isQuery = selectedAction.toLowerCase().includes('query');

      const res = await fetch(`http://localhost:4000/${isQuery ? 'query' : 'simulate'}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      const data = await res.json();
      console.log('✅ Received simulation result:', data);
      setLastSimulationTime(Date.now());
    } catch (err) {
      console.error('❌ Error reaching backend:', err);
    }
  };

  const handleFork = async () => {
    setForking(true);
    setForkStatus('');
    try {
      const res = await fetch('http://localhost:4000/fork', {
        method: 'POST',
      });
      const json = await res.json();
      setForkStatus(json.msg || 'Forked successfully!');
    } catch (err) {
      console.error('❌ Fork failed:', err);
      setForkStatus('Fork failed.');
    } finally {
      setForking(false);
    }
  };

  const schemaPath =
    selectedContract && selectedAction
      ? `${selectedContract}/${selectedAction}_msg.json`
      : '';

  console.log('🔍 Viewer input:', {
    selectedContract,
    selectedAction,
    lastSimulationTime,
  });

  if (loading) return <p>Loading available contracts...</p>;

  return (
    <div className="App">
      <h1>🧪 CosmWasm Simulation Dashboard</h1>

      <label>
        Select contract:&nbsp;
        <select value={selectedContract} onChange={handleContractChange}>
          {contractNames.map((contract) => (
            <option key={contract} value={contract}>
              {contract}
            </option>
          ))}
        </select>
      </label>

      <br />

      <label>
        Select action:&nbsp;
        <select
          value={selectedAction}
          onChange={handleActionChange}
          disabled={!selectedContract}
        >
          {(contracts[selectedContract] || []).map((action) => (
            <option key={action} value={action}>
              {action}
            </option>
          ))}
        </select>
      </label>

      <br />
      <button onClick={handleFork} disabled={forking}>
        {forking ? 'Forking...' : 'Fork Live State'}
      </button>
      {forkStatus && <p>{forkStatus}</p>}

      <h2>📤 Send Message</h2>
      {schemaPath && (
        <SchemaForm
          schemaPath={schemaPath}
          onSubmit={(msg) => handleExecuteSubmit(msg)}
        />
      )}

      <h2>🔁 Simulation Output</h2>
      <SimulationViewer
        contract={selectedContract}
        action={selectedAction}
        lastSimulationTime={lastSimulationTime}
      />
    </div>
  );
};

export default App;
