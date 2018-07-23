<?php

require_once(dirname(__FILE__) . '/library/config.php');

$nbt_file = getenv('nbt_file');
$program_name = getenv('program_name');
$problem_name = getenv('problem_name');

$problem_id = intval(Database::SelectCell('SELECT problem_id FROM problems WHERE problem_name = {problem_name}', ['problem_name' => $problem_name]));

if ($problem_id == 0) {
    WARNING("Failed to resolve problem name: $problem_name");
    exit(1);
}

$data = file_get_contents($nbt_file);

if ($data == '') {
    WARNING("NBT file is empty: $nbt_file");
    exit(1);
}

if (strlen($data) > 30000000) {
    WARNING("NBT file is too big: $nbt_file");
    exit(1);
}

Database::Command(
    'INSERT programs SET program_name = {program_name}',
    ['program_name' => $program_name]);
$program_id = Database::InsertId();

Database::Command('
    INSERT runs SET
        program_id = {program_id},
        problem_id = {problem_id},
        run_stdout = {run_stdout},
        run_score_queue = NOW() - INTERVAL 8 DAY',
    ['program_id' => $program_id,
     'problem_id' => $problem_id,
     'run_stdout' => $data]);
$run_id = Database::InsertId();

INFO('Successfully uploaded: https://icfpc-dashboard.appspot.com/run.php?run_id=' . $run_id);
