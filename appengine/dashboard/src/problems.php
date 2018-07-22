<?php

$problems = Database::Select(
	'SELECT problem_name, problem_resolution FROM problems');

foreach ($problems as $problem) {
	echo "<div style=\"display:inline-block; margin: 10px; border: 1px solid #888; \"><table style=\"border-collapse:collapse;border-spacing:0;width:400px;\"><tr><td style=\"padding:0\"><img src=\"thumbnails/{$problem['problem_name']}.png\" width=128 height=128></td><td style=\"padding: 10px;\">Name: {$problem['problem_name']}<br>Resolution: {$problem['problem_resolution']}</td></tr></table></div>";
}
