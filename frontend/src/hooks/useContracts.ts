import { useEffect, useState } from 'react';

export function useContracts() {
  const [contracts, setContracts] = useState<Record<string, string[]>>({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('http://localhost:4000/contracts')
      .then(res => res.json())
      .then(setContracts)
      .catch(err => console.error('❌ Failed to fetch contracts', err))
      .finally(() => setLoading(false));
  }, []);

  return { contracts, loading };
}
