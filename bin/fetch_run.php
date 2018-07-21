<?php

require_once(dirname(__FILE__) . '/library/config.php');

INFO('Fetching a run...');

Database::Command('
	UPDATE runs SET
		run_id = (@run_id := run_id),
		run_queue = NOW() + INTERVAL 10 SECOND
	WHERE run_queue < NOW() LIMIT 1');
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
	exit(0);
}

function FetchData($table, $column, $hash) {
	$paths = ["/efs/data/$hash", "/tmp/data/$hash"];
	foreach ($paths as $path) {
		if (is_readable($path)) {
			$data = file_get_contents($path);
			if (sha1($data) == $hash) {
				return $data;
			}
			@unlink($path);
		}
	}
	$data = Database::SelectCell("
		SELECT $column FROM $table WHERE {$column}_hash = {hash}",
		['hash' => $hash]);
	if (sha1($data) == $hash) {
		foreach ($paths as $path) {
			if (is_dir(dirname($path))) {
				file_put_contents($path, $data);
				return $data;
			}
		}
		WARNING("Failed to save cache.");
		return $data;
	}
	WARNING("Failed to fetch data: $hash from $table:$column");
	return NULL;
}

INFO("Preparing files...");
file_put_contents(
	'input', FetchData('problems', 'problem_data', $run['problem_data_hash']));
file_put_contents(
	'program', FetchData('programs', 'program_data', $run['program_data_hash']));
chmod('program', 0755);
file_put_contents('command', $run['program_command']);
file_put_contents('wrapper', '
{ time bash ./command | head -c 30000000 >stdout; } 2>&1 | head -c 1000000 >stderr
');

INFO("Executing a run (run_id={$run['run_id']})...");
$command = dirname(__FILE__) . '/timeout --alsologtostderr --timeout=30 bash ./wrapper';
echo $command;
system($command);

Database::Command('
	UPDATE runs
	SET
		run_stdout = {run_stdout},
		run_stderr = {run_stderr},
		run_executed = NOW(),
		run_queue = NULL
	WHERE run_id = @run_id', [
	'run_stdout' => file_get_contents('stdout'),
	'run_stderr' => file_get_contents('stderr'),
	'run_executed' => date('Y-m-d H:i:s'),
]);
