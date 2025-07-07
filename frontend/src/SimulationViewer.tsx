import React, { useEffect, useState } from 'react';

export const SimulationViewer = () => {
  const [data, setData] = useState<any>(null);
  const [timestamp, setTimestamp] = useState<number>(0);

  useEffect(() => {
    const fetchData = async () => {
      try {
       const res = await fetch('/simulations/latest_counter_increment.json', {
  cache: 'no-store',
});

        const json = await res.json();

        // Detect change (if 'count' or timestamp changed)
        const newCount = json.query_after?.count ?? 0;
        if (newCount !== data?.query_after?.count) {
          setData(json);
          setTimestamp(Date.now());
        }
      } catch (err) {
        console.error('❌ Failed to load simulation:', err);
      }
    };

    fetchData(); // initial fetch
    const interval = setInterval(fetchData, 5000); // poll every 5s
    return () => clearInterval(interval);
  }, [data]);

  return (
    <div>
      <h2>🔁 Simulation Output</h2>
      {data ? (
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
