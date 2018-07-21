<?php

$data = file_get_contents(
	'https://icfpcontest2018.github.io/lgtn/live-standings.html');

$data = explode('<h2 id="by-team">', $data, 2)[1];

list($teams, $data) = explode('</p>', $data, 2);

preg_match_all('%#team-(\d+)">([^<]*)</a>%Usi', $teams, $matches, PREG_SET_ORDER);
$programs = [];
foreach ($matches as $program) {
	$programs[] = [
		'program_id' => intval($program[1]),
		'program_name' => preg_replace('%^Team\s*%usi', '', $program[2]),
	];
}

echo '<pre>';
var_dump($programs);
Database::Command('REPLACE INTO programs{values}', ['values' => $programs]);

$problems = [];
foreach (Database::Select('SELECT problem_id, problem_name FROM problems') as $problem) {
	$problem_name = $problem['problem_name'];
	$problems[$problem_name] = $problem;
	$problem_name = preg_replace('%\.mdl$%', '', $problem_name);
	$problems[$problem_name] = $problem;
	$problem_name = preg_replace('%_tgt$%', '', $problem_name);
	$problems[$problem_name] = $problem;
	$problem_name = preg_replace('%^LA%', '', $problem_name);
	$problems[$problem_name] = $problem;
}

foreach (explode('<h3', $data) as $details) {
	if (!preg_match('%id="team-(\d+)">%', $details, $match)) {
		continue;
	}
	$values = [];
	$program_id = intval($match[1]);
	foreach (explode('<tr', $details) as $row) {
		if (!preg_match_all('%<pre>([^<]*)</pre>%Usi', $row, $matches)) {
			continue;
		}
		// print_r($matches);
		$matches = $matches[1];
		if (!isset($problems[$matches[0]])) {
			continue;
		}
		$problem = $problems[$matches[0]];
		$problem_id = $problem['problem_id'];
		$run_score = intval($matches[1]);
		$values[] = [
			'program_id' => $program_id,
			'problem_id' => $problem_id,
			'run_score' => $run_score,
		];
	}
	Database::Command('REPLACE INTO runs {values}', ['values' => $values]);
}

// print_r($matches);
// print_r($teams);
// print_r($data);
