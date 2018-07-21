<?php

require_once(dirname(__FILE__) . '/library/config.php');

$program_id = intval(getenv('PROGRAM_ID'));
INFO('Program ID: ' . $program_id . "\n");

$parameters = ['program_id' => $program_id];

$program = Database::SelectRow('
	SELECT program_id, program_name
	FROM programs WHERE program_id = {program_id}',
	$parameters);

if (is_null($program)) {
	WARNING("No such program ID: $program_id");
	exit(1);
}

Database::Command('
	INSERT INTO runs(problem_id, program_id, run_queue) SELECT
		problem_id,
		{program_id} AS program_id,
		NOW() AS run_queue
	FROM
		(SELECT problem_id FROM problems) AS s1
			NATURAL LEFT JOIN
		(SELECT problem_id, run_id
		 FROM runs WHERE program_id= {program_id}) AS s2
	WHERE run_id IS NULL', $parameters);
INFO("Affected rows: " . Database::AffectedRows());
