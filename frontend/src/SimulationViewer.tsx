// frontend/src/SimulationViewer.tsx
import React, { useEffect, useState } from 'react';

export const SimulationViewer = ({
  contract,
  action,
  lastSimulationTime,
}: {
  contract: string;
  action: string;
  lastSimulationTime: number;
}) => {
  const [data, setData] = useState<any>(null);
  const [timestamp, setTimestamp] = useState<number>(0);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      if (!contract || !action) {
        console.warn('⚠️ Missing contract or action, skipping fetch');
        return;
      }

     // const path = `/simulations/${contract}/${action}/result.json`;
     // …
      const path = `http://localhost:4000/simulations/${contract}/${action}/result.json`;
      console.log(`🔍 Fetching simulation result for [${contract}/${action}]`);
      console.log('📁 Full fetch path:', path);

      try {
        const res = await fetch(path, { cache: 'no-store' });

        if (!res.ok) {
          const text = await res.text();
          console.error(`❌ HTTP error ${res.status}: ${res.statusText}`);
          console.error('🚨 Response content:\n', text);

          setData(null);
          setError(
            `Failed to load result.json (status ${res.status}): ${res.statusText}`
          );
          return;
        }

        const json = await res.json();
        console.log('✅ Loaded simulation JSON:', json);

        setData(json);
        setTimestamp(Date.now());
        setError(null);
      } catch (err) {
        console.error('❌ Exception while fetching simulation result:', err);
        setData(null);
        setError(`Exception while fetching result: ${(err as Error).message}`);
      }
    };

    fetchData();
  }, [contract, action, lastSimulationTime]);

  return (
    <div>
      {error ? (
        <div style={{ color: 'red', fontWeight: 'bold' }}>
          ⚠️ {error}
        </div>
      ) : data ? (
        <pre style={{ background: '#f4f4f4', padding: '1em' }}>
          {JSON.stringify(data, null, 2)}
        </pre>
      ) : (
        <p>Loading simulation...</p>
      )}
      <small>Last updated: {new Date(timestamp).toLocaleTimeString()}</small>
    </div>
  );
};
