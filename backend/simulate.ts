// backend/simulate.ts
import { exec } from 'child_process';
import { writeFile, readFile } from 'fs/promises';
import path from 'path';

export async function runSimulation(msg: any) {
  const inputPath = path.resolve(__dirname, '../simulations/exec_msg.json');
  const outputPath = path.resolve(__dirname, '../frontend/public/simulations/latest_counter_increment.json');

  // Save input message
  await writeFile(inputPath, JSON.stringify(msg, null, 2));

  // Run Rust test
  await new Promise((resolve, reject) => {
    exec(
      'cargo test test_forked_counter_exec -- --nocapture',
      { cwd: path.resolve(__dirname, '..') },
      (err, stdout, stderr) => {
        if (err) {
          console.error(stderr);
          reject(err);
        } else {
          console.log(stdout);
          resolve(null);
        }
      }
    );
  });

  // Read output
  const output = await readFile(outputPath, 'utf-8');
  return JSON.parse(output);
}
