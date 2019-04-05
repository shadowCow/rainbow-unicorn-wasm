import { SimpleApp } from 'ru-wasm-simple-app';

const setupCanvas = () => {
  let canvas = document.getElementById("rainbow-unicorn-canvas");
  canvas.width = 800;
  canvas.height = 600;
};
setupCanvas();

const app = SimpleApp.new();

function animationLoop(timestamp) {
  app.tick(timestamp);

  window.requestAnimationFrame(animationLoop);
}

window.requestAnimationFrame(animationLoop);