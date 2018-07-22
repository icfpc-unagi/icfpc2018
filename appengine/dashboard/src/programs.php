<?php

ob_start();

$programs = Database::Select('
    SELECT program_id, program_name, program_command, program_created
    FROM programs
    ORDER BY program_id DESC');

function register_program() {
	if (!$_SERVER['USER_IS_ADMIN']) {
		return 'You are not admin.';
	}
	if (!$_POST['program_name']) {
		return 'Program name is missing.';
	}
	if (!$_POST['program_command']) {
		return 'Command is missing.';
	}
	if (strpos($_POST['program_command'], '${problem_name}')) {
		return 'Command does not contain ${problem_name}.';
	}
	Database::Command('INSERT IGNORE INTO programs {values}',
		['values' => [
			'program_name' => $_POST['program_name'],
			'program_command' => $_POST['program_command']]]);
	if (Database::AffectedRows() == 0) {
		return 'Failed to register (e.g., program name is duplicated).';
	}
	return FALSE;
}

if ($_POST['action'] == 'register_program') {
	$error = register_program();
	if ($error) {
		echo "<div class=\"error\">Error: $error</div>";
	} else {
		echo "<div class=\"success\">Successfully registered.</div>";
	}
}

echo '<h2>Register Program</h2>';


echo '<form class="form" action="programs.php" method="POST">';
echo '<input type="hidden" name="action" value="register_program">';
echo '<table width="100%">';
echo '<tr><td nowrap>Program Name: <td width="100%"><input type="text" name="program_name" value="' . htmlspecialchars($_POST['program_name']) . '">';
echo '<tr><td>Command: <td><input type="text" name="program_command" value="' . htmlspecialchars($_POST['program_command']) . '">';
echo '</table>';
echo '<center><input type="submit" value="Register Program"></center>';
echo '</form>';

echo '<h2>Programs</h2>';

echo '<table class="ranking">';
echo '<thead><td style="width:30%">Program Name</td><td style="width:70%">Command</td></thead>';

foreach ($programs as $program) {
    $program = array_map('htmlspecialchars', $program);
    echo "<tr><td><a href=\"/?program_id={$program['program_id']}\">{$program['program_name']}</a></td>";
    if ($program['program_command'] == '') {
    	echo '<td><i>No command</i></td>';
    } else {
	    echo "<td class=\"monospace\">{$program['program_command']}</td>";
	}
    echo "</tr>";
}

echo '</table>';

$body = ob_get_clean();
include('template.html');
