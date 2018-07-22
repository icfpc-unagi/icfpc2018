<?php



$problem_id = intval($_GET['problem_id']);
$problem = Database::SelectRow('
  SELECT problem_id, problem_name, problem_resolution, problem_has_target
  FROM problems
  WHERE problem_id = {problem_id}',
  ['problem_id' => $problem_id]);

if (is_null($problem)) {
  header('HTTP/1.1 404 Not Found');
  exit();
}

$programs = [];
foreach (Database::SELECT('
  SELECT program_id, program_name, run_score
  FROM runs NATURAL JOIN programs
  WHERE problem_id = {problem_id} AND run_score IS NOT NULL
  ORDER BY run_score',
  ['problem_id' => $problem_id]) as $run) {
  $programs[$run['program_id']] = $run;
}

ob_start();

?>
<h2>Problem: <?php echo htmlspecialchars($problem['problem_name']); ?> (R=<?php echo $problem['problem_resolution']; ?>, dfl=<?php echo $programs[9000]['run_score']; ?>)</h2>
<div style="margin: 1em; text-align: center">
<div id="glcanvas_container" style="position: relative; width: 600px; display: inline-block;">
<canvas id="glcanvas" tabindex="0" style="width:600px;height:400px" />
</div>
</div>

<script src="/assets/js/three.min.js"></script>
<script src="/assets/js/Detector.js"></script>
<script src="/assets/js/stats.min.js"></script>
<script src="/assets/js/visualizer.js"></script>
<script>
var vis;

function render(data) {
  var R = data[0];
  vis.setSize(R, R, R);
  for (var z = 0; z < R; z++) {
    for (var y = 0; y < R; y++) {
      for (var x = 0; x < R; x++) {
        var index = (x * R + y) * R + z;
        if (data[Math.floor(index / 8) + 1] & (1 << (index % 8))) {
          vis.fillMatrix(x, y, z, x, y, z);
        }
      }
    }
  }
  console.log(data);
}

(function () {
  if (! Detector.webgl) {
    const glcanvasContainer = document.getElementById('glcanvas_container');
    const glcanvas = document.getElementById('glcanvas');
    glcanvasContainer.removeChild(glcanvas);
    var warning = Detector.getWebGLErrorMessage();
    glcanvasContainer.appendChild(warning);
  } else {
    vis = initVisualizer({
      stats: true, screenshot: false, controls: true, noresize: true});
    var request = new XMLHttpRequest();
    request.open("GET", "/problems/<?php echo $problem['problem_name'] . ($problem['problem_has_target'] ? '_tgt.mdl' : '_src.mdl') ; ?>", true);
    request.responseType = "blob";
    request.onload = function() {
      var reader = new FileReader();
      reader.onload = function() {
        render(new Uint8Array(reader.result));
      };
      reader.readAsArrayBuffer(request.response);
    };
    request.send();
  }
})();
</script>
<h3>Ranking</h3>
<?php

echo '<table class="ranking">';
echo '<thead><td style="width:10%">Rank</td><td>Program Name</td><td style="width:20%">Energy</td><td style="width:10%">Score</td></thead>';
$best_score = array_values($programs)[0]['run_score'];
$default_score = $programs[9000]['run_score'];
$resolution = $problem['problem_resolution'];

$last_run_score = 0;
$index = 0;
foreach ($programs as $program) {
  $index++;
  $run_score = $program['run_score'];
  if ($last_run_score != $run_score) {
    $rank = $index;
    $last_run_score = $run_score;
  }
  $program = array_map('htmlspecialchars', $program);
  if ($default_score == $best_score) {
    $eval_score = floor(log($resolution) / log(2)) * 1000;
  } else {
    $eval_score = floor(
      (floor(log($resolution) / log(2)) * 1000 *
        ($default_score - $run_score)) /
      ($default_score - $best_score));
  }
  echo "<tr><td>$rank</td><td style=\"overflow-x:hidden; white-space: nowrap\">{$program['program_name']}</td><td style=\"text-align:right;\" class=\"monospace\">{$program['run_score']}</td><td style=\"text-align:right\">$eval_score</td></tr>";
}
echo '</table>';

$body = ob_get_clean();
include('template.html');
