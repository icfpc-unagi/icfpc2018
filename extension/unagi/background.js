const rules = [
  ["go", "icfpc-dashboard.appspot.com/go.php?"],
];

chrome.webNavigation.onBeforeNavigate.addListener((details) => {
  var url = details.url;
  for (var i in rules) {
    const rule = rules[i];
    if (url.startsWith("http://" + rule[0] + "/") ||
        url.startsWith("https://" + rule[0] + "/")) {
      url = url.replace("://" + rule[0] + "/", "://" + rule[1] + "/");
      url = url.replace("http://", "https://");
    }
  }
  if (details.url != url) {
    chrome.tabs.update(details.tabId, { url: url });
  }
}, {
  url: (function() {
    var prefixes = [];
    for (var i in rules) {
      const rule = rules[i];
      prefixes.push({ urlPrefix: "http://" + rule[0] + "/" });
      prefixes.push({ urlPrefix: "https://" + rule[0] + "/" });
    }
    return prefixes;
  }())
});
