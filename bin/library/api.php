<?php

function call_api($method, $data=NULL) {
    $options = [];
    $headers = ['User-Agent: unagi-api-from-' . getenv('USER')];
    if (!is_null($data)) {
        $data = http_build_query($data, '', '&');
        $headers[] = 'Content-Type: application/x-www-form-urlencoded';
        $headers[] = 'Content-Length: ' . strlen($data);
        $options['method'] = 'POST';
        $options['content'] = $data;
    }
    $headers[] = 'Authorization: Basic ' .
                 base64_encode('unagi:' . getenv('UNAGI_PASSWORD'));
    $options['header'] = implode("\r\n", $headers);
    $method = str_replace('.php', '', $method);
    $url = "https://icfpc-api.appspot.com/$method.php";
    $http_response_header = [];
    $output = @file_get_contents(
        $url, FALSE, stream_context_create(['http' => $options]));

    if (!preg_match('%HTTP/\d.\d 200 OK%i', $http_response_header[0])) {
        trigger_error(
            "API response's status code is invalid: {$http_response_header[0]}",
            E_USER_ERROR);
    }
    if ($output == '') {
        trigger_error("API call failed: $method", E_USER_ERROR);
    }
    return json_decode($output, TRUE);
}

if (basename(__FILE__) == basename($_SERVER['PHP_SELF'])) {
    var_dump(call_api('time'));
}
