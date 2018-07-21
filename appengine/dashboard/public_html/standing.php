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
	ORDER BY problem_id, run_score DESC');

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
	SELECT problem_id, problem_name
	FROM
		problems NATURAL JOIN
		(SELECT problem_id FROM standing GROUP BY problem_id) AS s') as $problem) {
	$problems[$problem['problem_id']] = $problem;
}

$standings = [];
foreach (Database::Select('SELECT * FROM standing') as $row) {
	$standings[$row['problem_id']][$row['program_id']] = $row;
}

echo '<h2 id="by-problem">By Problem</h2><p>';

foreach ($problems as $problem) {
	$problem_name = problem_name($problem['problem_name']);
	echo "<a href=\"#problem-$problem_name\">Problem $problem_name</a>　　";
}

echo '</p>';

foreach ($problems as $problem) {
	$problem_name = problem_name($problem['problem_name']);
	echo "<h3 id=\"problem-$problem_name\">Problem $problem_name</h3>\n";
	echo '
<table style="width:700px">
    <thead>
        <th style="width:50%">Team</th>
        <th style="width:30%">Total Energy</th>
        <th style="width:20%">Total Score</th>
    </thead>
    <tbody>';

    foreach ($standings[$problem['problem_id']] as $program) {
    	echo "<tr>
            <td style=\"text-align:left\"><pre>{$programs[$program['program_id']]['program_name']}</pre></td>
            <td style=\"text-align:right\"><pre>{$program['run_score']}</pre></td>
            <td style=\"text-align:right\"><pre>----</pre></td>
        </tr>";
    }

    echo '</tbody></table>';
}
$body = ob_get_clean();

include('standing.html');
