(async function() {
  let rustEnd = null;
  try {
    // import wasm
    rustEnd = await import('../pkg/index.js');
  } catch(e) {
    return console.log('error importing wasm:', e);
  }

  console.log('wasm import success:', rustEnd)

  const newImgTag = document.getElementById('new-img');
  const ip = document.getElementById('upload');
  const fReader = new FileReader();

  fReader.onloadend = _ => {
    // read base64 content and also remove the metadata that 
    // is prefixed to any base64 image data
    let b64Content = fReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ''
    );
    console.log('image reading completed', b64Content.slice(0, 20));
    // lets call the wasm function
    // this functions returns image base64 data url 
    // with grayscale effect applied
    b64Content = rustEnd.process_img(b64Content);
    newImgTag.setAttribute('src', b64Content);
  }

  ip.addEventListener('change', e => {
    fReader.readAsDataURL(ip.files[0]);
  })
})();
