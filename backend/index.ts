// backend/index.ts
import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import path from 'path';
import fs from 'fs';
import { runSimulation } from './simulate';

const app = express();
const PORT = 4000;

app.use(cors({
  origin: 'http://localhost:3000',
}));

app.use(bodyParser.json());

// simulation endpoint
app.post('/simulate', async (req, res) => {
  const msg = req.body;
  console.log('📨 Received simulation request:', msg);

  try {
    const result = await runSimulation(msg);
    res.json(result);
  } catch (err) {
    console.error('❌ Simulation failed:', err);
    res.status(500).json({ error: 'Simulation failed' });
  }
});

// endpoint to list available contracts and actions
app.get('/contracts', (req, res) => {
  const schemaRoot = path.join(process.cwd(), 'frontend/public/schema');
  console.log('📂 Reading schema from:', schemaRoot);

  const result: Record<string, string[]> = {};

  try {
    const contractDirs = fs.readdirSync(schemaRoot, { withFileTypes: true })
      .filter(dirent => dirent.isDirectory())
      .map(dirent => dirent.name);

    for (const contract of contractDirs) {
      const contractPath = path.join(schemaRoot, contract);
      const files = fs.readdirSync(contractPath)
        .filter(name => name.endsWith('_msg.json'));

      const actions = files.map(file => file.replace('_msg.json', ''));
      result[contract] = actions;
    }

    res.json(result);
  } catch (err) {
    console.error('❌ Failed to read schema directories', err);
    res.status(500).json({ error: 'Failed to load contracts' });
  }
});

// 🚀 Start server
app.listen(PORT, () => {
  console.log(`🚀 Backend running on http://localhost:${PORT}`);
});
