// backend/index.ts
import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import { runSimulation } from './simulate';

const app = express();
const PORT = 4000;

app.use(cors({
  origin: 'http://localhost:3000',
}));

app.use(bodyParser.json());

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

app.listen(PORT, () => {
  console.log(`🚀 Backend running on http://localhost:${PORT}`);
});
