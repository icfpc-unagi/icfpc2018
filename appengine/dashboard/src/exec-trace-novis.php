<?php

$run_id = @intval($_GET['run_id']);

$problem = Database::SelectRow('
  SELECT problem_id, problem_name, problem_has_source, problem_has_target
  FROM runs NATURAL JOIN problems WHERE run_id = {run_id}', ['run_id' => $run_id]);

print_r($problem);

?>
<!DOCTYPE html>
<html lang="en-US">
  <head>
    <meta charset='utf-8'>
    <meta http-equiv="X-UA-Compatible" content="chrome=1">
    <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1">
    <link href='https://fonts.googleapis.com/css?family=Architects+Daughter' rel='stylesheet' type='text/css'>
    <link rel="stylesheet" href="/assets/css/style.css?v=8a893d3ef4c1dbbf48fcc5b24f91772378143f2a" media="screen" type="text/css">
    <!-- <link rel="stylesheet" href="/assets/css/print.css" media="print" type="text/css"> -->

    <!--[if lt IE 9]>
    <script src="https://oss.maxcdn.com/html5shiv/3.7.3/html5shiv.min.js"></script>
    <![endif]-->

    <!-- Begin Jekyll SEO tag v2.5.0 -->
<title>Execute Trace (Full, No Visualizer) | ICFP Programming Contest 2018</title>
<meta name="generator" content="Jekyll v3.7.3" />
<meta property="og:title" content="Execute Trace (Full, No Visualizer)" />
<meta property="og:locale" content="en_US" />
<link rel="canonical" href="https://icfpcontest2018.github.io/full/exec-trace-novis.html" />
<meta property="og:url" content="https://icfpcontest2018.github.io/full/exec-trace-novis.html" />
<meta property="og:site_name" content="ICFP Programming Contest 2018" />
<script type="application/ld+json">
{"@type":"WebPage","url":"https://icfpcontest2018.github.io/full/exec-trace-novis.html","headline":"Execute Trace (Full, No Visualizer)","@context":"http://schema.org"}</script>
<!-- End Jekyll SEO tag -->

  </head>

  <body>
    <header>
      <div class="inner">
        <a href="https://icfpcontest2018.github.io/index.html">
          <h1>ICFP Programming Contest 2018</h1>
        </a>
        <!-- <h2>ICFP Programming Contest 2018 (website)</h2> -->
        
        
      </div>
    </header>


    <div id="content-wrapper">
      <div class="wide-inner clearfix">
        <section id="main-content" style="width: 100%;">
          <h1 id="execute-trace-full-no-visualizer">Execute Trace (Full, No Visualizer)</h1>

<form>

<input type="hidden" id="full" value="true" />

<p>
<label for="srcModelFileIn">Source Model:</label>
<input type="checkbox" id="srcModelEmpty" />
<label for="srcModelEmpty">empty</label><br />
<input type="file" accept=".mdl" id="srcModelFileIn" />
</p>

<p>
<label for="tgtModelFileIn">Target Model:</label>
<input type="checkbox" id="tgtModelEmpty" />
<label for="tgtModelEmpty">empty</label><br />
<input accept=".mdl" type="file" id="tgtModelFileIn" />
</p>

<p>
<label for="traceFileIn">Trace:</label>
<input accept=".nbt" type="file" id="traceFileIn" />
</p>


<p>
<label for="stepsPerFrame">Steps per Refresh:</label><br />
<select id="stepsPerFrame">
<option value="-60">1/60</option>
<option value="-30">1/30</option>
<option value="1">1</option>
<option value="100">100</option>
<option value="500">500</option>
<option value="1000">1000</option>
<option selected="" value="2000">2000</option>
<option value="3000">3000</option>
<option value="4000">4000</option>
</select>
</p>


<p>
<input type="button" id="execTrace" value="Execute Trace" disabled="" />
</p>

</form>

<hr />

<pre id="stdout"></pre>

<script>
var vis = null;
</script>

<script src="/assets/js/load-file-utils.js"></script>

<script>
var srcModelBData = null;
var tgtModelBData = null;
var traceBData = null;

var is_ready_functions = [];

var traceBDataIsReady = false;
(function () {
  var request = new XMLHttpRequest();
  request.open("GET", "/stdout.php?run_id=<?php echo $run_id; ?>", true);
  request.responseType = "blob";
  request.onload = function() {
    var reader = new FileReader();
    reader.onload = function() {
      traceBData = new Uint8Array(reader.result);
      traceBDataIsReady = true;
    };
    reader.readAsArrayBuffer(request.response);
  };
  request.send();
})();
is_ready_functions.push(function() { return traceBDataIsReady; });


<?php
if ($problem['problem_has_target']) {
?>
var tgtModelBDataIsReady = false;
(function() {
  var request = new XMLHttpRequest();
  request.open("GET", "/problems/<?php echo $problem['problem_name']; ?>_tgt.mdl", true);
  request.responseType = "blob";
  request.onload = function() {
    var reader = new FileReader();
    reader.onload = function() {
      tgtModelBData = new Uint8Array(reader.result);
      tgtModelBDataIsReady = true;
    };
    reader.readAsArrayBuffer(request.response);
  };
  request.send();
})();
is_ready_functions.push(function() { return tgtModelBDataIsReady; });
<?php
} else {
?>
document.getElementById("tgtModelEmpty").checked = true;
tgtModelBData = null;
<?php
}
?>

<?php
if ($problem['problem_has_source']) {
?>
var srcModelBDataIsReady = false;
(function() {
  var request = new XMLHttpRequest();
  request.open("GET", "/problems/<?php echo $problem['problem_name']; ?>_src.mdl", true);
  request.responseType = "blob";
  request.onload = function() {
    var reader = new FileReader();
    reader.onload = function() {
      srcModelBData = new Uint8Array(reader.result);
      srcModelBDataIsReady = true;
    };
    reader.readAsArrayBuffer(request.response);
  };
  request.send();
})();
is_ready_functions.push(function() { return srcModelBDataIsReady; });
<?php
} else {
?>
document.getElementById("srcModelEmpty").checked = true;
srcModelBData = null;
<?php
}
?>

function TryStart() {
  for (var i = 0; i < is_ready_functions.length; i++) {
    if (!is_ready_functions[i]()) {
      setTimeout(TryStart, 100);
      return;
    }
  }
  console.log("input is ready");
  document.getElementById("execTrace").click();
  setTimeout(CheckResult, 100);
}

function CheckResult() {
  var stdout = document.getElementById("stdout").innerText;
  if (!stdout.match("Failure") && !stdout.match("Success")) {
    setTimeout(CheckResult, 100);
    return;
  }
  console.log("result is ready");
  if (stdout.match("Success::")) {
    console.log(stdout.match(/Energy:\s*(\d+)/)[1]);
    location.href="/exec_result.php?run_id=<?php echo $run_id; ?>&result="+
                  stdout.match(/Energy:\s*(\d+)/)[1];
  } else {
    console.log("failure");
    location.href="/exec_result.php?run_id=<?php echo $run_id; ?>&result=-1";
  }
}

setTimeout(TryStart, 100);

(function () {
  var srcModelEmpty = document.getElementById('srcModelEmpty');
  var tgtModelEmpty = document.getElementById('tgtModelEmpty');
  var execTrace = document.getElementById('execTrace');
  execTrace.disabled = false;
  function onStart() {
    document.getElementById('stdout').innerHTML = "";
    if (vis) { vis.setSize(8, 8, 8); };
    execTrace.disabled = true;
  }
  function onSuccess() {
    if ((srcModelBData || srcModelEmpty.checked) &&
        (tgtModelBData || tgtModelEmpty.checked) &&
        !(srcModelEmpty.checked && tgtModelEmpty.checked) &&
        traceBData) {
      execTrace.disabled = false;
    }
  }
  // mkLoadBDataFromFile
  // ('srcModel',
  //  function () { srcModelBData = null; onStart(); },
  //  function () { },
  //  onSuccess,
  //  function(data) { srcModelBData = data; });
  // document.getElementById('srcModelEmpty').addEventListener('change',
  //   function (e) { onStart();
  //                  if (e.target.checked) {
  //                    srcModelBData = null;
  //                    document.getElementById('srcModelFileIn').disabled = true;
  //                    document.getElementById('srcModelFileIn').value = "";
  //                  } else {
  //                    document.getElementById('srcModelFileIn').disabled = false;
  //                  }
  //                  onSuccess();
  //   }, false);
  // mkLoadBDataFromFile
  // ('tgtModel',
  //  function () { tgtModelBData = null; onStart(); },
  //  function () { },
  //  onSuccess,
  //  function(data) { _tgtModelBData = data; });
  // document.getElementById('tgtModelEmpty').addEventListener('change',
  //   function (e) { onStart();
  //                  if (e.target.checked) {
  //                    tgtModelBData = null;
  //                    document.getElementById('tgtModelFileIn').disabled = true;
  //                    document.getElementById('tgtModelFileIn').value = "";
  //                  } else {
  //                    document.getElementById('tgtModelFileIn').disabled = false;
  //                  }
  //                  onSuccess();
  //   }, false);
  // mkLoadBDataFromFile
  // ('trace',
  //  function () { traceBData = null; onStart(); },
  //  function () { },
  //  onSuccess,
  //  function(data) { _traceBData = data; });
})();
</script>

<script src="/assets/js/exec-trace.js"></script>


        </section>
      </div>
    </div>

    

  </body>
</html>

