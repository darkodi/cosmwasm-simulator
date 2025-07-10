// backend/index.ts
import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import path from 'path';
import fs from 'fs';
import { runSimulation } from './simulate';
import { runQuery } from './simulate';
import { exec } from 'child_process';

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

app.post('/query', async (req, res): Promise<void> => {
  const { contract, action } = req.body;
  console.log(`📨 Running query for [${contract}/${action}]`);

  try {
    const result = await runQuery(contract, action);
    res.json(result);
  } catch (err) {
    console.error('❌ Query failed:', err);
    res.status(500).json({ error: 'Query failed' });
  }
});

// ───────────── fork endpoint ─────────────
app.post('/fork', async (req, res) => {
  console.log('🔁 Forking live state from chain...');
  try {
    const basePath = path.resolve(__dirname, '..');
    const outputPath = path.join(basePath, 'frontend/public/simulations/cw_tpl_osmosis/query/result.json');

    await new Promise((resolve, reject) => {
      exec(
        'cargo test test_fork_live_state -- --nocapture',
        {
          cwd: basePath,
          env: {
            ...process.env,
            SIMULATION_QUERY_OUTPUT_PATH: outputPath,
          },
        },
        (err: Error | null, stdout: string, stderr: string) => {
          if (err) {
            console.error('❌ Forking error:', stderr);
            reject(err);
          } else {
            console.log('✅ Forking done:\n', stdout);
            resolve(null);
          }
        }
      );
    });

    res.status(200).json({ status: 'ok', msg: 'Forked state from chain' });
  } catch (err) {
    res.status(500).json({ status: 'error', msg: 'Forking failed', error: String(err) });
  }
});



// ───────────── start server ─────────────
app.listen(PORT, () => {
  console.log(`🚀 Backend running on http://localhost:${PORT}`);
});
