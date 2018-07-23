<?php

$run_id = intval($_GET['run_id']);
$run = Database::SelectRow('
  SELECT run_id, run_stdout, program_name, problem_name
  FROM runs NATURAL JOIN programs NATURAL JOIN problems
  WHERE run_id = {run_id}',
  ['run_id' => $run_id]);

if (is_null($run['run_stdout'])) {
  header('HTTP/1.1 404 Not Found');
  exit();
}

header('Content-Disposition: attachment; filename="' . $run['program_name'] . '_' . $run['problem_name'] . '.nbt"');
header('Content-Type: application/octet-stream');
echo $run['run_stdout'];
