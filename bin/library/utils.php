<?php

function FetchData($table, $column, $hash) {
	$paths = ["/efs/data/$hash", "/tmp/data/$hash"];
	foreach ($paths as $path) {
		if (is_readable($path)) {
			$data = file_get_contents($path);
			if (sha1($data) == $hash) {
				return $data;
			}
			@unlink($path);
		}
	}
	$data = Database::SelectCell("
		SELECT $column FROM $table WHERE {$column}_hash = {hash}",
		['hash' => $hash]);
	if (sha1($data) == $hash) {
		foreach ($paths as $path) {
			if (is_dir(dirname($path))) {
				file_put_contents($path, $data);
				return $data;
			}
		}
		WARNING("Failed to save cache.");
		return $data;
	}
	WARNING("Failed to fetch data: $hash from $table:$column");
	return NULL;
}
