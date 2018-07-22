<?php

$program_id = @intval($_GET['program_id']);

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

$where = '';

if ($_GET['filter'] == 'public') {
    $where .= ' AND problem_is_public ';
}

if ($_GET['group'] == 'fa') {
    $where .= ' AND problem_name LIKE "FA%" ';
}

if ($_GET['group'] == 'fd') {
    $where .= ' AND problem_name LIKE "FD%" ';
}

if ($_GET['group'] == 'fr') {
    $where .= ' AND problem_name LIKE "FR%" ';
}

Database::Command("
    CREATE TEMPORARY TABLE standing AS
    SELECT
        run_id, program_id, problem_id,
        run_score, best_run_score, default_run_score,
        (CASE WHEN best_run_score = default_run_score THEN
            FLOOR(LOG2(problem_resolution)) * 1000
        ELSE
            FLOOR(
                FLOOR(LOG2(problem_resolution)) * 1000 *
                (default_run_score - run_score) /
                (default_run_score - best_run_score))
        END) AS eval_score
    FROM
        runs NATURAL LEFT JOIN
        (SELECT problem_id, IFNULL(run_score, 0) AS default_run_score
         FROM runs NATURAL RIGHT JOIN problems
         WHERE program_id = 9000) AS default_run_scores
            NATURAL LEFT JOIN
        (SELECT problem_id, MIN(run_score) AS best_run_score
         FROM runs NATURAL JOIN problems
         GROUP BY problem_id) AS best_run_scores
            NATURAL LEFT JOIN
        problems
    WHERE run_score IS NOT NULL $where
    ORDER BY problem_id, run_score ASC");

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
       SELECT problem_id, problem_name, problem_resolution, problem_has_target, problem_has_source
       FROM
               problems NATURAL JOIN
               (SELECT problem_id FROM standing GROUP BY problem_id) AS s
       ORDER BY problem_name') as $problem) {
    $problems[$problem['problem_id']] = $problem;
}

$standings = [];
foreach (Database::Select('SELECT * FROM standing') as $row) {
    $standings[$row['problem_id']][$row['program_id']] = $row;
}

if (!$program_id) {
    $stats = Database::SelectRow('
        SELECT
            SUM(run_score_queue < NOW() - INTERVAL 1 WEEK) AS score_queue_week,
            SUM(run_score_queue < NOW() - INTERVAL 1 DAY) AS score_queue_day,
            SUM(run_score_queue < NOW() - INTERVAL 1 HOUR) AS score_queue_hour,
            SUM(run_score_queue < NOW()) AS score_queue,
            SUM(run_score_queue >= NOW()) AS score_queue_lock,
            SUM(run_queue < NOW() - INTERVAL 1 WEEK) AS run_queue_week,
            SUM(run_queue < NOW() - INTERVAL 1 DAY) AS run_queue_day,
            SUM(run_queue < NOW() - INTERVAL 1 HOUR) AS run_queue_hour,
            SUM(run_queue < NOW()) AS run_queue,
            SUM(run_queue >= NOW()) AS run_queue_lock,
            SUM(run_executed > NOW() - INTERVAL 60 SECOND) AS executed_1m,
            SUM(run_executed > NOW() - INTERVAL 600 SECOND) AS executed_10m,
            SUM(run_executed > NOW() - INTERVAL 1 HOUR) AS executed_1h,
            SUM(run_executed > NOW() - INTERVAL 1 DAY) AS executed_1d
        FROM runs');
    $stats = array_map('intval', $stats);

    // buggy : 1 day : normal : 1 week : emergency
    foreach (['score_queue', 'run_queue'] as $queue) {
        $stats["{$queue}_emergency"] = $stats["{$queue}_week"];
        $stats["{$queue}_normal"] =
            $stats["{$queue}_day"] - $stats["{$queue}_week"];
        $stats["{$queue}_buggy"] = $stats[$queue] - $stats["{$queue}_day"];
    }

    echo '<h2>Stats</h2><ul>';
    echo "<li>Executions: {$stats['executed_1m']} in 1 minute, {$stats['executed_10m']} in 10 minutes, {$stats['executed_1h']} in 1 hour, {$stats['executed_1d']} in 1 day\n";
    echo "<li>Execution queue: running={$stats['run_queue_lock']}, queued={$stats['run_queue']} (emergency={$stats['run_queue_emergency']}, normal={$stats['run_queue_normal']}, buggy={$stats['run_queue_buggy']})\n";
    echo "<li>Scoring queue: running={$stats['score_queue_lock']}, queued={$stats['score_queue']} (emergency={$stats['score_queue_emergency']}, normal={$stats['score_queue_normal']}, buggy={$stats['score_queue_buggy']})\n";
    echo '</ul>';
}

if ($program_id >= 10000) {
    echo '<h2>Execution</h2>';

    function enqueue() {
        global $program_id;
        if (!$_SERVER['USER_IS_ADMIN']) {
            return 'You are not admin.';
        }
        $program_group = $_POST['problem_group'];
        if ($program_group == 'small') {
            $pattern = '%001';
        } else {
            $pattern = "$program_group%";
        }
        Database::Command('
            INSERT IGNORE INTO runs(problem_id, program_id, run_queue) SELECT
                problem_id,
                {program_id} AS program_id,
                NOW() - INTERVAL (RAND() + 1) * 24 * 60 * 60 SECOND AS run_queue
            FROM problems
            WHERE problem_name LIKE {pattern}',
            ['pattern' => $pattern, 'program_id' => $program_id]);
    }

    if ($_POST['action'] == 'enqueue') {
        $error = enqueue();
        if ($error) {
            echo "<div class=\"error\">Error: $error</div>";
        } else {
            echo "<div class=\"success\">Successfully queued.</div>";
        }
    }

    function rescore() {
        global $program_id;
        if (!$_SERVER['USER_IS_ADMIN']) {
            return 'You are not admin.';
        }
        Database::Command('
            UPDATE runs SET
                run_score_queue =
                    NOW() - INTERVAL (RAND() + 14) * 24 * 60 * 60 SECOND
            WHERE run_stdout IS NOT NULL AND run_score_queue IS NULL AND
                program_id = {program_id}',
            ['program_id' => $program_id]);
    }

    if ($_POST['action'] == 'rescore') {
        $error = rescore();
        if ($error) {
            echo "<div class=\"error\">Error: $error</div>";
        } else {
            echo "<div class=\"success\">Successfully queued.</div>";
        }
    }

    echo '<div class="form"><center>';

    ob_start();
    $incomplete = FALSE;
    foreach (Database::Select('
        SELECT
            LEFT(problem_name, 2) AS problem_group,
            SUM(waiting) AS waiting,
            SUM(running) AS running,
            SUM(complete) AS complete,
            COUNT(*) AS total
        FROM
            (
            SELECT
                problem_id,
                run_queue <= NOW() AS waiting,
                run_queue > NOW() AS running,
                run_stdout IS NOT NULL AS complete
            FROM
                runs
            WHERE
                program_id = {program_id}
        ) AS s
        NATURAL RIGHT JOIN problems GROUP BY problem_group',
        ['program_id' => $program_id]) as $problem_group) {
        switch ($problem_group['problem_group']) {
            case 'FA': $name = 'Assemble'; break;
            case 'FD': $name = 'Disassemble'; break;
            case 'FR': $name = 'Reassemble'; break;
            default: $name = $problem_group['problem_group']; break;
        }
        if ($problem_group['total'] == $problem_group['complete']) {
            $disabled = ' disabled';
        } else {
            $disabled = '';
            $incomplete = TRUE;
        }
        echo "<form action=\"/?program_id=$program_id\" method=\"POST\" style=\"display:inline-block; margin: 0 10px;\">";
        echo '<input type="hidden" name="action" value="enqueue">';
        echo "<input type=\"hidden\" name=\"problem_group\" value=\"{$problem_group['problem_group']}\">";
        echo "<center><input type=\"submit\" value=\"Start $name\" $disabled></center>";
        echo '</form>';
    }
    $group_output = ob_get_clean();

    echo "<form action=\"/?program_id=$program_id\" method=\"POST\" style=\"display:inline-block; margin: 0 10px;\">";
    echo '<input type="hidden" name="action" value="enqueue">';
    echo "<input type=\"hidden\" name=\"problem_group\" value=\"\">";
    $disabled = $incomplete ? '' : ' disabled';
    echo "<center><input type=\"submit\" value=\"Start All\" $disabled></center>";
    echo '</form>';

    echo "<form action=\"/?program_id=$program_id\" method=\"POST\" style=\"display:inline-block; margin: 0 10px;\">";
    echo '<input type="hidden" name="action" value="enqueue">';
    echo "<input type=\"hidden\" name=\"problem_group\" value=\"small\">";
    $disabled = $incomplete ? '' : ' disabled';
    echo "<center><input type=\"submit\" value=\"Start Small\" $disabled></center>";
    echo '</form>';

    echo $group_output;

    echo "<form action=\"/?program_id=$program_id\" method=\"POST\" style=\"display:inline-block; margin: 0 10px;\">";
    echo '<input type="hidden" name="action" value="rescore">';
    echo "<input type=\"hidden\" name=\"problem_group\" value=\"{$problem_group['problem_group']}\">";
    echo "<center><input type=\"submit\" value=\"Restart Scoring\"></center>";
    echo '</form>';

    echo '</center></div>';
}

$num_ranks = 10;
echo '<h2>Overeview</h2>';

echo '<form action="/" method="GET">';
if ($_GET['program_id']) {
    echo '<input type="hidden" name="program_id" value="' .
        $_GET['program_id'] . '">';
}

echo '<select name="filter">';
echo '<option value="" ' .
     ($_GET['filter'] == '' ? ' selected' : '') . '>';
echo 'All problems</option>';
echo '<option value="public" ' .
     ($_GET['filter'] == 'public' ? ' selected' : '') . '>';
echo 'Public only</option>';
echo '</select>';

echo '<select name="group">';
echo '<option value="" ' .
     ($_GET['group'] == '' ? ' selected' : '') . '>';
echo 'All types</option>';
echo '<option value="fa" ' .
     ($_GET['group'] == 'fa' ? ' selected' : '') . '>';
echo 'Assemble only</option>';
echo '<option value="fd" ' .
     ($_GET['group'] == 'fd' ? ' selected' : '') . '>';
echo 'Disassemble only</option>';
echo '<option value="fr" ' .
     ($_GET['group'] == 'fr' ? ' selected' : '') . '>';
echo 'Reassemble only</option>';
echo '</select>';

echo '<input type="submit" value="View" style="margin:0 10px">';
echo '</form>';

echo '<div style="width:100%;overflow-x:scroll"><table class="table">';
echo '<thead><td style="width:250px">Problem</td>';

if ($program_id) {
    echo "<td style=\"width:120px;white-space:nowrap;overflow-x:hidden\">{$programs[$program_id]['program_name']}</td>";

    $runs = [];
    foreach (Database::Select('
        SELECT
            run_id,
            program_id,
            problem_id,
            run_score,
            run_score_queue,
            run_score_stdout IS NOT NULL AS run_score_stdout,
            run_stdout IS NOT NULL AS run_stdout,
            run_queue,
            run_executed,
            run_modified,
            run_created
        FROM runs
        WHERE program_id = {program_id}', ['program_id' => $program_id]) as $run) {
        $runs[$run['problem_id']] = $run;
    }
}

function to_rank($num) {
    switch ($num) {
        case 1: return '🥇 1st';
        case 2: return '🥈 2nd';
        case 3: return '🥉 3rd';
    }
    if ($num % 10 == 1 && $num > 20) return $num . 'st';
    if ($num % 10 == 2 && $num > 20) return $num . 'nd';
    if ($num % 10 == 3 && $num > 20) return $num . 'rd';
    return $num . 'th';
}

for ($i = 1; $i <= $num_ranks; $i++) {
    echo '<td style="width:120px">' . to_rank($i) . '</td>';
}
echo '</thead>';

$total_rankings = [];
$rank = 0;
$my_rank = 'Unknown';
foreach (Database::Select('SELECT program_id, SUM(eval_score) AS total_score FROM standing GROUP BY program_id ORDER BY total_score DESC') as $program) {
    $rank++;
    if ($program['program_id'] == $program_id) {
        $my_rank = to_rank($rank);
    }
    $total_rankings[$program['program_id']] = $program;
}

echo '<tr><td>Total</td>';

if ($program_id) {
    echo '<td class="rank">' . $my_rank . '<br>' . $total_rankings[$program_id]['total_score'] . '</td>';
}

$total_rankings = array_values($total_rankings);
for ($i = 0; $i < $num_ranks; $i++) {
    $program = $total_rankings[$i];
    $program += $programs[$program['program_id']];
    echo '<td class="rank">';
    if ($program['program_id'] == $program_id) {
        echo '<b>';
    } else {
        echo '<a href="/?program_id=' . $program['program_id'] . '">';
    }
    echo $program['program_name'];
    if ($program['program_id'] == $program_id) {
        echo '</b>';
    } else {
        echo '</a>';
    }
    echo '<br>' . $program['total_score'] . '</td>';
}

echo '</tr>';

foreach ($problems as $problem) {
    echo '<tr>';
    $problem_name = problem_name($problem['problem_name']);
    $resolution = $problem['problem_resolution'];
    $default = @$standings[$problem['problem_id']][9000];
    $default_score = sprintf('%.2e', $default['run_score']);
    echo "<td style=\"padding:0\"><span style=\"display:inline-block; height: 96px; vertical-align: middle;\">";
    if ($problem['problem_has_target']) {
        echo "<img src=\"/thumbnails/{$problem_name}_tgt.mdl.png\" width=96 height=96>";
    } else {
        echo "<img src=\"/thumbnails/{$problem_name}_src.mdl.png\" width=96 height=96>";
    }
    echo "</span><span style=\"display:inline-block; vertical-align: middle; padding: 5px;\"><a href=\"/problem.php?problem_id={$problem['problem_id']}\">{$problem_name}</a><br>R={$problem['problem_resolution']}<br>dfl=$default_score</span></td>";

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
    $default_score = floatval($default['run_score']);
    for ($i = $program_id ? -1 : 0; $i < $num_ranks; $i++) {
        if ($i == -1) {
            $program = @$standings[$problem['problem_id']][$program_id];
        } else {
            $program = @$ranked_programs[$i];
        }
        if (!$program) {
            if ($i == -1) {
                $run = $runs[$problem['problem_id']];
                $score = $run['run_score'];
                if (is_null($score)) {
                    if (!is_null($run['run_queue'])) {
                        if ($run['run_queue'] < date('Y-m-d H:i:s')) {
                            $score = 'Waiting';
                        } else {
                            $score = 'Running';
                        }
                    } else if ($run['run_score_stdout']) {
                        $score = 'Error';
                    } else if ($run['run_stdout']) {
                        $score = 'Scoring';
                    } else {
                        $score = 'Disabled';
                    }
                }
                echo "<td>";
                if ($run['run_id']) {
                    echo "<a href=\"/run.php?run_id={$run['run_id']}\">";
                }
                echo "<i>$score</i>";
                if ($run['run_id']) {
                    echo "</a>";
                }
                echo "</td>";
                continue;
            }
            echo '<td class="rank"></td>';
            continue;
        }
        $my_score = $program['run_score'];
        $d = @($default_score / $my_score);
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
        if ($program['program_id'] >= 5000) {
            echo "<a href=\"/run.php?run_id={$program['run_id']}\">";
        }
        echo "{$program['run_score']}";
        if ($program['program_id'] >= 5000) {
            echo "</a>";
        }
        echo "<br>=dfl/$d<br>{$program['eval_score']}</td>";
    }
    echo '</tr>';
}

echo '</table></div>';

$body = ob_get_clean();
include('template.html');
