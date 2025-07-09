// backend/index.ts
import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import path from 'path';
import fs from 'fs';
import { runSimulation } from './simulate';

const app = express();
const PORT = 4000;

app.use(
  cors({
    origin: 'http://localhost:3000',
  })
);
app.use(bodyParser.json());

/* ──────────────────────────────────────────────────────────────
   Serve result.json files that the Rust test writes
   URL example: http://localhost:4000/simulations/cw_tpl_osmosis/reset/result.json
──────────────────────────────────────────────────────────────── */
app.use(
  '/simulations',
  express.static(path.join(__dirname, '../frontend/public/simulations'))
);

// ───────────── simulation endpoint ─────────────
app.post('/simulate', async (req, res): Promise<void> => {
  const { contract, action, msg } = req.body;
  console.log(`📨 Simulating [${contract}/${action}] with msg:`, msg);

  try {
    const result = await runSimulation(msg, contract, action);
    res.json(result);
  } catch (err) {
    console.error('❌ Simulation failed:', err);
    res.status(500).json({ error: 'Simulation failed' });
  }
});

// ───────────── contracts list endpoint ─────────────
app.get('/contracts', (req, res) => {
  const schemaRoot = path.resolve(__dirname, '../frontend/public/schema');
  console.log('📂 Reading schema from:', schemaRoot);

  const result: Record<string, string[]> = {};

  try {
    const contractDirs = fs
      .readdirSync(schemaRoot, { withFileTypes: true })
      .filter((dirent) => dirent.isDirectory())
      .map((dirent) => dirent.name);

    for (const contract of contractDirs) {
      const contractPath = path.join(schemaRoot, contract);
      const files = fs
        .readdirSync(contractPath)
        .filter((name) => name.endsWith('_msg.json'));

      result[contract] = files.map((f) => f.replace('_msg.json', ''));
    }

    res.json(result);
  } catch (err) {
    console.error('❌ Failed to read schema directories', err);
    res.status(500).json({ error: 'Failed to load contracts' });
  }
});

// ───────────── start server ─────────────
app.listen(PORT, () => {
  console.log(`🚀 Backend running on http://localhost:${PORT}`);
});
