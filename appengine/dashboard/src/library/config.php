<?php

error_reporting(E_ERROR | E_WARNING | E_PARSE);

date_default_timezone_set('Asia/Tokyo');

require_once(dirname(__FILE__) . '/database.php');

Database::Initialize(
	'master.cbr6mhuodbur.ap-northeast-1.rds.amazonaws.com', 3306,
	'unagi', getenv('UNAGI_PASSWORD'));

ini_set('memory_limit', '1G');
