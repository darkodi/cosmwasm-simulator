// backend/simulate.ts
import { exec } from 'child_process';
import { writeFile, readFile } from 'fs/promises';
import path from 'path';

export async function runSimulation(msg: any, contractName: string, actionName: string) {
  const basePath = path.resolve(__dirname, '..');
  const inputPath = path.join(basePath, `simulations/${contractName}/${actionName}/exec_msg.json`);
  const outputPath = path.join(basePath, `frontend/public/simulations/${contractName}/${actionName}/result.json`);

  await writeFile(inputPath, JSON.stringify({
    msg,
    output_path: outputPath,
  }, null, 2));

  await new Promise((resolve, reject) => {
    exec(
      'cargo test test_forked_counter_exec -- --nocapture',
      {
        cwd: basePath,
        env: {
          ...process.env,
          SIMULATION_INPUT_PATH: inputPath, //injects dynamic path
        },
      },
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

  const output = await readFile(outputPath, 'utf-8');
  return JSON.parse(output);
}
