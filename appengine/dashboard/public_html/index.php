<?php

ob_start();

function problem_name($problem_name) {
	return strtr($problem_name, ['_tgt.mdl' => '']);
}

Database::Command('
	CREATE TEMPORARY TABLE standing AS
	SELECT program_id, problem_id, run_score
	FROM
		(SELECT program_id, problem_id, MAX(run_score) AS run_score
		 FROM runs GROUP BY program_id, problem_id) AS s
	WHERE run_score IS NOT NULL
	ORDER BY problem_id, run_score ASC');

$programs = [];
foreach (Database::Select('
	SELECT program_id, program_name
	FROM
		programs NATURAL JOIN
		(SELECT program_id FROM standing GROUP BY program_id) AS s
	ORDER BY program_name DESC') as $program) {
	$programs[$program['program_id']] = $program;
}

$problems = [];
foreach (Database::Select('
	SELECT problem_id, problem_name, problem_resolution
	FROM
		problems NATURAL JOIN
		(SELECT problem_id FROM standing GROUP BY problem_id) AS s') as $problem) {
	$problems[$problem['problem_id']] = $problem;
}

$standings = [];
foreach (Database::Select('SELECT * FROM standing') as $row) {
	$standings[$row['problem_id']][$row['program_id']] = $row;
}

$num_ranks = 10;
echo '<h2>Overeview</h2>';
echo '<div style="width:100%;overflow-x:scroll"><table class="table">';
echo '<thead><td style="width:250px">Problem</td>';
for ($i = 1; $i <= $num_ranks; $i++) {
	echo '<td style="width:120px">';
	switch ($i) {
		case 1: echo '🥇 1st'; break;
		case 2: echo '🥈 2nd'; break;
		case 3: echo '🥉 3rd'; break;
		default: echo $i . 'th'; break;
	}
	echo '</td>';
}
echo '</thead>';

foreach ($problems as $problem) {
	echo '<tr>';
	$problem_name = problem_name($problem['problem_name']);
	$resolution = $problem['problem_resolution'];
	$default = $standings[$problem['problem_id']][9000];
	$default_score = sprintf('%.2e', $default['run_score']);
	echo "<td style=\"padding:0\"><span style=\"display:inline-block; height: 96px; vertical-align: middle;\"><img src=\"/thumbnails/{$problem_name}_tgt.mdl.png\" width=96 height=96></span><span style=\"display:inline-block; vertical-align: middle; padding: 5px;\"><a href=\"/problem.php?problem_id={$problem['problem_id']}\">{$problem_name}</a><br>R={$problem['problem_resolution']}<br>dfl=$default_score</span></td>";

	$ranked_programs = array_values($standings[$problem['problem_id']]);
	$best_score = $ranked_programs[0]['run_score'];
	$default_score = intval($default['run_score']);
	for ($i = 0; $i < $num_ranks; $i++) {
		if (!isset($ranked_programs[$i])) {
			echo '<td class="rank"></td>';
			continue;
		}
		$program = $ranked_programs[$i];
		$my_score = $program['run_score'];
		if ($default_score == $best_score) {
			$eval_score = floor(log($resolution) / log(2)) * 1000;
		} else {
			$eval_score = floor(
				(floor(log($resolution) / log(2)) * 1000 *
					($default_score - $my_score)) /
				($default_score - $best_score));
		}
		$d = $default_score / $my_score;
		if ($d < 10) {
			$d = sprintf('%.2f', $d);
		} else if ($d < 100) {
			$d = sprintf('%.1f', $d);
		} else {
			$d = round($d);
		}
		$percent = sprintf('%.4f%%', $my_score / $default_score * 100);
    	echo "<td class=\"rank\">{$programs[$program['program_id']]['program_name']}<br>{$program['run_score']}<br>=dfl/$d<br>$eval_score</td>";
	}
    // $count = 0;
    // foreach ($standings[$problem['problem_id']] as $program) {
    // 	if ($count >= 5) break;
    // 	$count++;
    // 	echo "<td class=\"rank\">{$programs[$program['program_id']]['program_name']}<br>{$program['run_score']}</td>";
    // }

    echo '</tr>';
}

echo '</table></div>';

$body = ob_get_clean();
include('template.html');
