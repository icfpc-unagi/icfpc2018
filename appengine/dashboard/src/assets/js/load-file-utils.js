function bdataLength (data) {
    return data.length;
}
function bdataSub (data, i) {
    return data[i];
}
function mkLoadBDataFromFile(id, onStart, onError, onSuccess, setData) {
   function loadFile(e) {
      onStart();
      var file = e.target.files[0];
      if (!file) { onError(); return; }
      var reader = new FileReader();
      reader.onload = function(e) {
         setData(new Uint8Array(e.target.result));
         onSuccess();
      };
      reader.readAsArrayBuffer(file);
   }
   document.getElementById(id + 'FileIn').addEventListener('change', loadFile, false);
}

function mkLoadTDataFromFile(id, onStart, onError, onSuccess, setData) {
   function loadFile(e) {
      onStart();
      var file = e.target.files[0];
      if (!file) { onError(); return; }
      var reader = new FileReader();
      reader.onload = function(e) {
         setData(e.target.result);
         onSuccess();
      };
      reader.readAsText(file);
   }
   document.getElementById(id + 'FileIn').addEventListener('change', loadFile, false);
}
