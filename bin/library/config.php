<?php

date_default_timezone_set('Asia/Tokyo');

if (php_sapi_name() != 'cli' &&
	!preg_match('%Development Server%', $_SERVER['SERVER_SOFTWARE'])) {
	if (!isset($_SERVER['PHP_AUTH_USER']) and
		isset($_SERVER['HTTP_AUTHORIZATION'])) {
		$arr = explode(" ", $_SERVER['HTTP_AUTHORIZATION']);
		$arr = explode(":", base64_decode($arr[1]));
		$_SERVER['PHP_AUTH_USER'] = $arr[0];
		$_SERVER['PHP_AUTH_PW']   = $arr[1];
	}

	if (!isset($_SERVER['PHP_AUTH_USER']) ||
		!($_SERVER['PHP_AUTH_USER'] == 'unagi' &&
		  $_SERVER['PHP_AUTH_PW'] == getenv('UNAGI_PASSWORD'))) {
	    header('WWW-Authenticate: Basic realm="Require UNAGI_PASSWORD."');
	    header('HTTP/1.0 401 Unauthorized');
	    exit();
	}
}

require_once(dirname(__FILE__) . '/database.php');

Database::Initialize(
	'master.cbr6mhuodbur.ap-northeast-1.rds.amazonaws.com', 3306,
	'unagi', getenv('UNAGI_PASSWORD'));

ini_set('memory_limit', '1G');

function Logging($color, $message) {
  fwrite(STDERR,
         $color . date('Y-m-d H:i:s] ') . rtrim($message) . "\033[0m\n");
}

function INFO($message) {
  Logging("\033[0;34mI", $message);
}

function WARNING($message) {
  Logging("\033[0;31mW", $message);
}
