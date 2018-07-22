<?php

ob_start();

$run_id = intval($_GET['run_id']);
$run = Database::SelectRow('
  SELECT run_id, problem_id, program_name, run_score, run_score_stdout, run_score_stderr, run_score_queue, run_stderr, run_queue, run_executed, run_modified, run_created
  FROM runs NATURAL JOIN problems NATURAL JOIN programs
  WHERE run_id = {run_id}',
  ['run_id' => $run_id]);

if (is_null($run)) {
  header('HTTP/1.1 404 Not Found');
  exit();
}

echo "<pre>";
print_r($run);
echo "</pre>";

$body = ob_get_clean();
include('template.html');
