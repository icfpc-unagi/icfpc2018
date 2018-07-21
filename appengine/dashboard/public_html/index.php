<?php

$program_id = intval($_GET['program_id']);

ob_start();

Database::Command('SET time_zone = "Asia/Tokyo"');

function problem_name($problem_name) {
	return strtr($problem_name, ['_tgt.mdl' => '']);
}

if ($program_id) {
	$program = Database::SelectRow('SELECT program_id, program_name, program_command, program_created FROM programs WHERE program_id = {program_id}', ['program_id' => $program_id]);
	echo '<h2>Program: ' . $program['program_name'] . '</h2>';
	echo '<ul class="monospace"><li>Program ID: ' . $program['program_id'];
	echo '<li>Program Name: ' . $program['program_name'];
	echo '<li>Program Command: ' . htmlspecialchars($program['program_command']);
	echo '<li>Creation time: ' . $program['program_created'];
	echo '</ul>';
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

if ($program_id) {
	echo "<td style=\"width:120px;white-space:nowrap;overflow-x:hidden\">{$programs[$program_id]['program_name']}</td>";
}

function to_rank($num) {
	switch ($num) {
		case 1: return '🥇 1st';
		case 2: return '🥈 2nd';
		case 3: return '🥉 3rd';
		default: return $num . 'th';
	}
}

for ($i = 1; $i <= $num_ranks; $i++) {
	echo '<td style="width:120px">' . to_rank($i) . '</td>';
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
	$my_rank = 'Unknown';
	if ($program_id) {
		$last_run_score = 0;
		$index = 0;
		foreach ($ranked_programs as $program) {
			$index++;
			$run_score = $program['run_score'];
			if ($last_run_score != $run_score) {
				$rank = $index;
				$last_run_score = $run_score;
			}
			if ($program['program_id'] == $program_id) {
				$my_rank = to_rank($rank);
			}
		}
	}
	$best_score = $ranked_programs[0]['run_score'];
	$default_score = intval($default['run_score']);
	for ($i = $program_id ? -1 : 0; $i < $num_ranks; $i++) {
		if ($i == -1) {
			$program = $standings[$problem['problem_id']][$program_id];
		} else {
			$program = $ranked_programs[$i];
		}
		if (!$program) {
			echo '<td class="rank"></td>';
			continue;
		}
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

    	if ($i == -1) {
	    	echo "<td class=\"rank\">";
	    	echo "$my_rank<br>";
	    } else {
	    	if ($program['program_id'] == $program_id) {
		    	echo "<td class=\"rank\" style=\"background-color:rgba(255,0,0,0.2)\">";
		    	echo '<b>';
	    	} else {
		    	echo "<td class=\"rank\">";
		    	echo "<a href=\"/?program_id={$program['program_id']}\">";
		    }
	    	echo $programs[$program['program_id']]['program_name'];
	    	if ($program['program_id'] == $program_id) {
	    		echo '</b>';
	    	} else {
		    	echo "</a>";
		    }
		    echo '<br>';
	    }
    	echo "{$program['run_score']}<br>=dfl/$d<br>$eval_score</td>";
	}
    echo '</tr>';
}

echo '</table></div>';

$body = ob_get_clean();
include('template.html');
