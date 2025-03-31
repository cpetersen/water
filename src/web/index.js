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
      try {
        // Calculate grid position
        const gridX = Math.floor(mouseX / canvas.width * GRID_SIZE);
        const gridY = Math.floor(mouseY / canvas.height * GRID_SIZE);
        
        // Make sure grid coordinates are valid
        if (gridX >= 0 && gridX < GRID_SIZE && gridY >= 0 && gridY < GRID_SIZE) {
          // Add density at mouse position (use a smaller value to prevent overflow)
          fluidSim.add_density(gridX, gridY, 0.5);
          
          // Add velocity based on mouse movement (with clamping to reasonable values)
          const dx = Math.max(-10, Math.min(10, mouseX - lastMouseX));
          const dy = Math.max(-10, Math.min(10, mouseY - lastMouseY));
          fluidSim.add_velocity(gridX, gridY, dx * 0.05, dy * 0.05);
        }
        
        lastMouseX = mouseX;
        lastMouseY = mouseY;
      } catch (error) {
        console.error("Error in mouse interaction:", error);
      }
    }
  });

  // Animation loop
  function animate() {
    try {
      // Clear canvas
      ctx.fillStyle = 'black';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      
      // Step simulation
      fluidSim.step();
      
      // Render fluid
      fluidSim.render(ctx, canvas);
    } catch (error) {
      console.error("Error in animation loop:", error);
      // Continue the animation loop even if there's an error
    }
    
    requestAnimationFrame(animate);
  }

  animate();
});