<?php

require_once(dirname(__FILE__) . '/library/config.php');

$problem_file = preg_replace('%(_tgt|_src)\.mdl%', '', getenv('PROBLEM'));
preg_match('%([^/_]*)(?:_[^/]*|)$%', $problem_file, $match);
$problem_name = $match[1];
if (is_readable($problem_file . '_src.mdl')) {
	$problem_has_source = True;
	$problem_data = file_get_contents($problem_file . '_src.mdl');
}
if (is_readable($problem_file . '_tgt.mdl')) {
	$problem_has_target = True;
	$problem_data = file_get_contents($problem_file . '_tgt.mdl');
}
$problem_resolution = ord($problem_data[0]);
$data_size = ceil(pow($problem_resolution, 3) / 8);
// $problem_data_hash = sha1($problem_data);
$problem_is_extended = (strlen($problem_data) > $data_size + 1);

echo "Problem file: $problem_file\n";
echo "Problem name: $problem_name\n";
echo "Problem has source: $problem_has_source\n";
echo "Problem has target: $problem_has_target\n";
echo "Problem file size: " . strlen($problem_data) . "\n";
echo "Problem resolution: $problem_resolution\n";
echo "Problem is extended: " . ($problem_is_extended ? "YES" : "NO") . "\n";
echo "Problem data size: " . $data_size . "\n";
// echo "Problem data hash: " . $problem_data_hash . "\n";

if ($problem_resolution <= 0) {
	trigger_error("Invalid problem resolution", E_USER_ERROR);
}
if (strlen($problem_data) < $data_size + 1) {
	trigger_error("Problem data is incomplete", E_USER_ERROR);
}

Database::Command('INSERT INTO problems {values}', ['values' => [
	'problem_name' => $problem_name,
	'problem_resolution' => $problem_resolution,
	'problem_has_source' => (bool)$problem_has_source,
	'problem_has_target' => (bool)$problem_has_target,
]]);
