<?php

require_once(dirname(__FILE__) . '/library/config.php');

INFO('Fetching a run...');

foreach (Database::Select('
	SELECT problem_id, MIN(run_score) AS run_score
	FROM runs NATURAL JOIN problems
	WHERE run_stdout IS NOT NULL AND
	      run_score IS NOT NULL
	GROUP BY problem_id ORDER BY problem_name') as $problem) {
	$problem = Database::SelectRow('
		SELECT
			run_id, problem_id, run_score,
			problem_name, program_name,
			COMPRESS(run_stdout) AS run_stdout
		FROM runs NATURAL JOIN problems NATURAL JOIN programs
		WHERE problem_id = {problem_id} AND run_score = {run_score}',
		$problem);
	$problem_name = str_replace('_tgt.mdl', '', $problem['problem_name']);
	INFO("Solution for problem: $problem_name is made by program: {$problem['program_name']}.");
	file_put_contents(
		$problem_name . '.nbt',
		gzuncompress(substr($problem['run_stdout'], 4)));
}
