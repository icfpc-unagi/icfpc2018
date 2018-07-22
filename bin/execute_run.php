<?php

require_once(dirname(__FILE__) . '/library/config.php');

$run_id = intval(getenv('run_id'));

INFO('Fetching a run...');

if ($run_id) {
	Database::Command('SET @run_id := {run_id}', ['run_id' => $run_id]);
} else {
	Database::Command('
		UPDATE runs SET
			run_id = (@run_id := run_id),
			run_queue = NOW() + INTERVAL 2000 SECOND
		WHERE run_queue < NOW() LIMIT 1');
}

$run = Database::SelectRow('
	SELECT
		run_id,
		problem_name,
		problem_data_hash,
		program_name,
		program_command,
		program_data_hash
	FROM runs NATURAL JOIN programs NATURAL JOIN problems
	WHERE run_id = @run_id');

if (is_null($run)) {
	INFO('Nothing to run.');
    sleep(1);
    exit();
}

INFO("Preparing files...");
file_put_contents(
	'input', file_get_contents(dirname(__FILE__) . '/../data/problemsF/' . $run['problem_name'] . '_tgt.mdl'));
file_put_contents('command',
	'problem_name=' . escapeshellarg($run['problem_name']) . "\n" .
	$run['program_command']);
file_put_contents('wrapper', '
{ time bash ./command | head -c 30000000 >stdout; } 2>&1 | { head -c 1000000; cat >/dev/null; } >stderr
');

INFO("Executing a run (run_id={$run['run_id']})...");
$command = dirname(__FILE__) . '/timeout --timeout=1800 bash ./wrapper';
system($command);

if ($run_id) {
	print_r([
		'run_stdout' => filesize('stdout'),
		'run_stderr' => filesize('stderr'),
		'run_executed' => date('Y-m-d H:i:s'),
	]);
	exit();
}

Database::Command('
	UPDATE runs
	SET
		run_stdout = {run_stdout},
		run_stderr = {run_stderr},
		run_executed = NOW(),
		run_queue = NULL,
		run_score_queue = NOW() - INTERVAL 1 DAY
	WHERE run_id = @run_id', [
	'run_stdout' => file_get_contents('stdout'),
	'run_stderr' => file_get_contents('stderr'),
	'run_executed' => date('Y-m-d H:i:s'),
]);
