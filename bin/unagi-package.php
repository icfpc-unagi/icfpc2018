<?php

require_once(dirname(__FILE__) . '/library/config.php');

INFO('Fetching a run...');

foreach (Database::Select('
	SELECT problem_id, problem_name, MIN(run_score) AS run_score,
		MIN(official_score) AS official_score
	FROM runs NATURAL JOIN problems NATURAL LEFT JOIN
		(SELECT run_id, official_score
		 FROM official_scores WHERE official_score > 0) AS s
	WHERE run_stdout IS NOT NULL AND
	      run_score IS NOT NULL
	GROUP BY problem_id ORDER BY problem_name') as $problem) {
	$original_problem = $problem;
	$problem = Database::SelectRow('
		SELECT
			run_id, problem_id, official_score,
			problem_name, program_name,
			COMPRESS(run_stdout) AS run_stdout
		FROM runs NATURAL JOIN problems NATURAL JOIN
			official_scores NATURAL JOIN programs
		WHERE problem_id = {problem_id} AND official_score = {official_score}
		LIMIT 1',
		$problem);
	$problem_name = str_replace('_tgt.mdl', '', $problem['problem_name']);
	INFO("Solution for problem ({$problem['official_score']}): $problem_name is made by program: {$problem['program_name']}.");
	if ($problem['official_score'] != $original_problem['run_score']) {
		WARNING("Official score for {$original_problem['problem_name']} is not best: official_score ({$problem['official_score']}) != run_score ({$original_problem['run_score']})");
	}
	file_put_contents(
		$problem_name . '.nbt',
		gzuncompress(substr($problem['run_stdout'], 4)));
}
