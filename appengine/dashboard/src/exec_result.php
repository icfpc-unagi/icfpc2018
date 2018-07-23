<?php

$run_id = @intval($_GET['run_id']);
$result = $_GET['result'];

if ($run_id) {
	Database::Command('
	  REPLACE INTO official_scores SET official_score = {result}, run_id = {run_id}',
	  ['run_id' => $run_id, 'result' => $result]);
}

Database::Command('
	UPDATE official_scores SET run_id = (@run_id := run_id), official_score_queue = NOW() + INTERVAL 3600 SECOND WHERE official_score_queue < NOW()
	ORDER BY official_score_queue LIMIT 1');

$run_id = Database::SelectCell('SELECT @run_id');

header('Location: /exec-trace-novis.php?run_id=' . $run_id);
