function init() {
  const input = document.getElementById('upload');
  const fileReader = new FileReader();

  fileReader.onload = (e) => {
    let base64 = fileReader.result.replace(
      /^data:image\/(png|jpg|jpeg);base64,/,
      ''
    );
    rustApp.grayscale(base64);
  };

  input.addEventListener('change', () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
