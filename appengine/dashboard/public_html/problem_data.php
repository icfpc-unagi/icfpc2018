<?php

$problem_data = Database::SelectCell('
	SELECT problem_data FROM problems WHERE problem_id = {problem_id}',
	['problem_id' => intval($_GET['problem_id'])]);

if (is_null($problem_data)) {
	header('HTTP/1.1 404 Not Found');
	exit();
}

header('Content-Type: application/octet-stream');
echo $problem_data;
