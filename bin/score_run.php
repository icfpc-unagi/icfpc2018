<?php

require_once(dirname(__FILE__) . '/library/config.php');

INFO('Fetching a run for scoring...');

$dryrun = @(bool)getenv('DRYRUN');
$run_id = @intval(getenv('RUN_ID'));
$simulator_binary = getenv('SIMULATOR_BINARY') ?: 'sim';

if ($run_id > 0) {
    Database::Command('SET @run_id := {run_id}', ['run_id' => $run_id]);
} else {
    Database::Command('
        UPDATE runs SET
            run_id = (@run_id := run_id),
            run_score_queue = NOW() + INTERVAL ' . ($dryrun ? 1 : 100) . ' SECOND
        WHERE run_score_queue < NOW()
        ORDER BY run_score_queue LIMIT 1');
}
$run = Database::SelectRow('
    SELECT
        run_id,
        problem_name,
        problem_has_source,
        problem_has_target,
        run_stdout
    FROM runs NATURAL JOIN problems
    WHERE run_id = @run_id');

if (is_null($run)) {
    INFO('Nothing to run.');
    sleep(1);
    exit();
}

INFO("Preparing files...");
file_put_contents('assembly', $run['run_stdout']);
$command = $simulator_binary . ' -a assembly';
if ($run['problem_has_source']) {
    file_put_contents(
        'source', file_get_contents(dirname(__FILE__) . '/../data/problemsF/' . $run['problem_name'] . '_src.mdl'));
    $command .= ' -s source';
}
if ($run['problem_has_target']) {
    file_put_contents(
        'target', file_get_contents(dirname(__FILE__) . '/../data/problemsF/' . $run['problem_name'] . '_tgt.mdl'));
    $command .= ' -t target';
}
file_put_contents('command', $command);
file_put_contents('wrapper', '
{ time bash ./command | head -c 30000000 >stdout; } 2>&1 | { head -c 1000000; cat >/dev/null; } >stderr
');

INFO("Scoring a run (run_id={$run['run_id']})...");
$command = dirname(__FILE__) . '/timeout --timeout=600 bash ./wrapper';
system($command);

$stdout = file_get_contents('stdout');
$energy = NULL;
if (preg_match('%^energy:(\d+)$%Usim', $stdout, $match)) {
    $energy = intval($match[1]);
}

if ($dryrun) {
    print_r([
        'run_score_stdout' => $stdout,
        'run_score_stderr' => file_get_contents('stderr'),
        'score' => $energy,
    ]);
    exit(0);
}

Database::Command('
    UPDATE runs
    SET
        run_score_stdout = {run_score_stdout},
        run_score_stderr = {run_score_stderr},
        run_score = CASE WHEN {score} = "" THEN NULL ELSE {score} END,
        run_score_queue = NULL
    WHERE run_id = @run_id', [
    'run_score_stdout' => $stdout,
    'run_score_stderr' => file_get_contents('stderr'),
    'score' => $energy,
]);
