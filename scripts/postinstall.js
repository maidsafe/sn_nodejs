const { exec } = require('child_process');

if (process.env.SAFE_NODEJS_DEV) {
    console.log('Skipped downloading node abi');
} else {
  exec('yarn run download-node-abi', (err, stdout, stderr) => {
    if (err) {
      // node couldn't execute the command
      return;
    }

    // the *entire* stdout and stderr (buffered)
    console.log(`stdout: ${stdout}`);
    console.log(`stderr: ${stderr}`);
  });
}
