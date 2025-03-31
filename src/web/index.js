// Import the WASM module
import init, { FluidSimulation } from '../../pkg/water';

// Initialize the WASM module
init().then(() => {
  // Canvas setup
  const canvas = document.getElementById('fluid-canvas');
  const ctx = canvas.getContext('2d');

  // Set canvas size to match window size
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  // Create fluid simulation
  const GRID_SIZE = 100;
  const fluidSim = FluidSimulation.new(GRID_SIZE, GRID_SIZE, 0.0001, 0.0000001, 0.2);

  // Mouse tracking
  let mouseX = 0;
  let mouseY = 0;
  let lastMouseX = 0;
  let lastMouseY = 0;
  let isMouseDown = false;

  canvas.addEventListener('mousedown', (e) => {
    isMouseDown = true;
    lastMouseX = mouseX = e.clientX;
    lastMouseY = mouseY = e.clientY;
  });

  canvas.addEventListener('mouseup', () => {
    isMouseDown = false;
  });

  canvas.addEventListener('mousemove', (e) => {
    mouseX = e.clientX;
    mouseY = e.clientY;
    
    if (isMouseDown) {
      // Calculate grid position
      const gridX = Math.floor(mouseX / canvas.width * GRID_SIZE);
      const gridY = Math.floor(mouseY / canvas.height * GRID_SIZE);
      
      // Add density at mouse position
      fluidSim.add_density(gridX, gridY, 100);
      
      // Add velocity based on mouse movement
      const dx = mouseX - lastMouseX;
      const dy = mouseY - lastMouseY;
      fluidSim.add_velocity(gridX, gridY, dx * 0.2, dy * 0.2);
      
      lastMouseX = mouseX;
      lastMouseY = mouseY;
    }
  });

  // Animation loop
  function animate() {
    // Clear canvas
    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    // Step simulation
    fluidSim.step();
    
    // Render fluid
    fluidSim.render(ctx, canvas);
    
    requestAnimationFrame(animate);
  }

  animate();
});