<?php

ob_start();

echo '<h2>Problems</h2>';

echo '<center>';

$problems = Database::Select(
	'SELECT problem_name, problem_resolution, problem_has_source, problem_has_target FROM problems WHERE problem_name LIKE "F%"');

foreach ($problems as $problem) {
	echo '<div style="display:inline-block; margin: 5px; border: 3px solid #000; border-radius: 10px; position: relative;"><table style="height:128px; border-collapse: collapse; padding: 0; margin: 0;"><tr>';
	$width = 0;
	if ($problem['problem_has_source']) {
		$width++;
	?>
<td style="background: url(thumbnails/<?php echo $problem['problem_name']; ?>_src.mdl.png); background-size: 128px 128px; width: 128px; height: 128px; box-sizing: border-box; color: white; text-align: center; vertical-align: top"></td>
	<?php
	}
	if ($problem['problem_has_target']) {
		$width++;
	?>
<td style="background: url(thumbnails/<?php echo $problem['problem_name']; ?>_tgt.mdl.png); background-size: 128px 128px; width: 128px; height: 128px; box-sizing: border-box; color: white; text-align: center; vertical-align: top"></td>
	<?php
	}
	// echo "<div style=\"display:inline-block; margin: 10px; border: 1px solid #888; \"><table style=\"border-collapse:collapse;border-spacing:0;width:400px;\"><tr><td style=\"padding:0\"><img src=\"thumbnails/{$problem['problem_name']}.png\" width=128 height=128></td><td style=\"padding: 10px;\">Name: {$problem['problem_name']}<br>Resolution: {$problem['problem_resolution']}</td></tr></table></div>";
	echo '</tr></table>';

	if ($problem['problem_has_source']) {
		if ($problem['problem_has_target']) {
			$mode = '⚒';
		} else {
			$mode = '💥';
		}
	} else {
		$mode = '🏢';
	}

	echo '<div style="position:absolute;left:0;top:0;width:' . ($width * 128) . 'px;color:#fff;text-align:center;font-size:90%;">' . $mode . $problem['problem_name'] . ' (R:' . $problem['problem_resolution'] . ')</div>';
	echo '</div>';
}

echo '</center>';

$body = ob_get_clean();
include('template.html');
