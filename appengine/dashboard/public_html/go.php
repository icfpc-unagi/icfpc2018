<?php


$links = [
	[['home', 'doc', 'docs'], 'https://docs.google.com/document/d/1lmea7iyfzY2xTYFCgkshPiS4xfhLo4dSWKtd9utAbbk/edit'],
	['slack', 'https://icfpc-unagi.slack.com'],
	[['web', 'blog'], 'https://icfpcontest2018.github.io'],
	['twitter', 'https://twitter.com/ICFPContest2018'],
	['mail', 'https://groups.google.com/forum/#!forum/icfpc-unagi'],
	[['drive', 'folder', 'directory'], 'https://drive.google.com/open?id=1NEALWily-zUHbiyUcf1VazV9VaGx50j6'],
	['dropbox', 'https://www.dropbox.com/sh/4cva7arxu53gj92/AABCUEuXAHT00mOsnnSJxUOOa'],
	[['github', 'git'], 'https://github.com/imos/icfpc2018'],
	[['issues', 'issue'], 'https://github.com/imos/icfpc2018/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc'],
	[['phpmyadmin', 'db', 'sql', 'mysql'], 'https://icfpc-phpmyadmin.appspot.com/db_structure.php?server=1&db=unagi'],
];

$mappings = [];
foreach ($links as $pair) {
	list($names, $url) = $pair;
	if (!is_array($names)) {
		$names = [$names];
	}
	foreach ($names as $name) {
		$mappings[$name] = $url;
	}
}


function Parse() {
	$link = isset($_SERVER['QUERY_STRING']) ? $_SERVER['QUERY_STRING'] : '';
	preg_match('%^/?([^/?]*)(.*)$%', $link, $match);
	return [$match[1], $match[2]];
}


function BaseUrl($name) {
	global $mappings;
	if (isset($mappings[$name])) {
		return $mappings[$name];
	}
	if ($name == '') {
		global $links;
		echo '<html><header><meta charset="UTF-8"></header><body><ul>';
		foreach ($links as $pair) {
			list($names, $url) = $pair;
			if (!is_array($names)) {
				$names = [$names];
			}
			$names = array_map('htmlspecialchars', $names);
			$url = htmlspecialchars($url);
			echo '<li>';
			$first = TRUE;
			foreach ($names as $name) {
				if ($first) {
					$first = FALSE;
				} else {
					echo ' or ';
				}
				echo "<a href=\"$url\">$name</a>";
			}
			echo " … $url";
		}
		echo '</ul></body></html>';
		exit();
	}
	header('HTTP/1.1 404 Not Found');
	echo "Unknown name: $name";
	exit();
}


function Redirect($url) {
	header('HTTP/1.1 301 Moved Permanently');
	header('Location: ' . $url);
}


list($name, $trailing) = Parse();
$base_url = BaseUrl($name);
$url = $base_url . $trailing;
Redirect($url);
