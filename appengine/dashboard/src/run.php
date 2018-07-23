<?php

ob_start();

$run_id = intval($_GET['run_id']);
$run = Database::SelectRow('
  SELECT run_id, problem_id, program_name, run_time_limit, run_score, run_score_stdout, run_score_stderr, run_score_queue, run_stdout IS NOT NULL AS run_stdout, run_stderr, run_queue, run_executed, run_modified, run_created
  FROM runs NATURAL JOIN problems NATURAL JOIN programs
  WHERE run_id = {run_id}',
  ['run_id' => $run_id]);

if (is_null($run)) {
  header('HTTP/1.1 404 Not Found');
  exit();
}

function increase_time_limit() {
	global $run_id;
	if (!$_SERVER['USER_IS_ADMIN']) {
		return 'You are not admin.';
	}
	Database::Command('
		UPDATE runs
		SET run_time_limit = 24 * 60 * 60,
			run_queue = NOW() - INTERVAL 2 WEEK
		WHERE run_id = {run_id}',
  		['run_id' => $run_id]);
	return FALSE;
}

function rejudge() {
	global $run_id;
	if (!$_SERVER['USER_IS_ADMIN']) {
		return 'You are not admin.';
	}
	Database::Command('
		UPDATE runs
		SET run_queue = NOW() - INTERVAL 2 WEEK
		WHERE run_id = {run_id}',
  		['run_id' => $run_id]);
	return FALSE;
}

if ($_POST['action'] == 'increase_time_limit') {
	$error = increase_time_limit();
	if ($error) {
		echo "<div class=\"error\">Error: $error</div>";
	} else {
		echo "<div class=\"success\">Successfully queued.</div>";
	}
} else if ($_POST['action'] == 'rejudge') {
	$error = rejudge();
	if ($error) {
		echo "<div class=\"error\">Error: $error</div>";
	} else {
		echo "<div class=\"success\">Successfully queued.</div>";
	}
} else if (!$run['run_queue'] && !$run['run_score']) {
	echo '<h2>Rejudge</h2>';

	echo '<div class="form">';

	echo "<form action=\"run.php?run_id=$run_id\" method=\"POST\">";
	echo '<input type="hidden" name="action" value="rejudge">';
	echo '<center><input type="submit" value="Redjuge"></center>';
	echo '</form>';

	echo "<form action=\"run.php?run_id=$run_id\" method=\"POST\">";
	echo '<input type="hidden" name="action" value="increase_time_limit">';
	echo '<center><input type="submit" value="Redjuge forever"></center>';
	echo '</form>';

	echo '</div>';
}

if ($run['run_stdout']) {
	echo "<a href=\"stdout.php?run_id=$run_id\">Download Output</a>";
}

echo "<pre>";
print_r($run);
echo "</pre>";

$body = ob_get_clean();
include('template.html');
