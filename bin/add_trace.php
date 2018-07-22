<?php

require_once(dirname(__FILE__) . '/library/config.php');

$nbt_file = getenv('NBT_FILE');

if (!is_readable($nbt_file)) {
	WARNING('No such file: ' . $nbt_file);
	exit(1);
}


$problem_name = preg_replace('%^[^/]*/%', '', $nbt_file);
$problem_name = str_replace('.nbt', '', $problem_name);

$problem_id = Database::SelectCell('SELECT problem_id FROM problems WHERE problem_name = {problem_name}', ['problem_name' => $problem_name]);

echo "Problem ID: " . $problem_id . "\n";

if (Database::SelectCell('SELECT run_stdout IS NOT NULL FROM runs WHERE program_id = 9000 AND problem_id = {problem_id}', ['problem_id' => $problem_id])) {
	INFO("Data exists");
	exit(0);
}

$data = file_get_contents($nbt_file);
echo "Data size: " . strlen($data) . "\n";

Database::Command('
	REPLACE INTO runs SET
		program_id = 9000,
		problem_id = {problem_id},
		run_score_queue = NOW(),
		run_stdout = {run_stdout}',
	['problem_id' => $problem_id,
	 'run_stdout' => $data]);
