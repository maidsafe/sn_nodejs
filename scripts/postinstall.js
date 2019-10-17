const { exec } = require('child_process');

if (process.env.SAFE_NODEJS_DEV) {
    console.log('Skipped downloading node abi');
} else {
	const useTravisWaitEnhanced = process.env.USE_TRAVIS_WAIT_ENHANCED;
	let command = 'yarn run download-node-abi';

	if( useTravisWaitEnhanced )
		command = `travis-wait-enhanced ${command}`;

  exec( command, (err, stdout, stderr) => {
    if (err) {
      // node couldn't execute the command
      return;
    }

    // the *entire* stdout and stderr (buffered)
    console.log(`stdout: ${stdout}`);
    console.log(`stderr: ${stderr}`);
  });
}
