// Import the WASM module
import init, { FluidSimulation } from '../../pkg/water';

// Configurable simulation parameters
const CONFIG = {
  // Grid and simulation settings
  gridSize: 100,               // Grid resolution (higher values = more detailed but slower)
  diffusion: 0.0001,           // How quickly the fluid diffuses (higher = more diffusion)
  viscosity: 0.0000001,        // Fluid thickness (higher = more viscous/thicker)
  timeStep: 0.2,               // Simulation time step (higher = faster but less stable)
  
  // Interaction settings
  densityAmount: 5.0,          // Amount of density added per interaction (higher = more visible)
  velocityScale: 0.05,         // How strongly mouse movements affect the fluid
  
  // Fluid decay
  densityDecay: 0.999,         // Rate at which density fades (1.0 = no decay, 0.9 = fast decay)
  velocityDecay: 0.99,         // Rate at which velocity slows down
  
  // Visual settings
  fluidColor: [0, 100, 255],   // RGB values for fluid color
  backgroundColor: [0, 0, 0],  // RGB values for background
  colorIntensity: 0.5,         // Intensity multiplier for colors
  
  // Effect settings
  responsive: true,            // Whether to resize with the window
  showVelocity: false,         // Show velocity vectors (for debugging)
  
  // Performance settings
  frameSkip: 0                 // Skip N frames between calculations (0 = calculate every frame)
};

// Initialize the WASM module
init().then(() => {
  // Canvas setup
  const canvas = document.getElementById('fluid-canvas');
  const ctx = canvas.getContext('2d');

  // Set canvas size based on settings
  if (CONFIG.responsive) {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    
    // Add resize listener
    window.addEventListener('resize', () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    });
  } else {
    // Use the size from CSS or set a default
    canvas.width = canvas.clientWidth || 800;
    canvas.height = canvas.clientHeight || 600;
  }

  // Create fluid simulation with configurable parameters
  const fluidSim = FluidSimulation.new(
    CONFIG.gridSize,
    CONFIG.gridSize, 
    CONFIG.diffusion,
    CONFIG.viscosity, 
    CONFIG.timeStep
  );

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
        const gridX = Math.floor(mouseX / canvas.width * CONFIG.gridSize);
        const gridY = Math.floor(mouseY / canvas.height * CONFIG.gridSize);
        
        // Make sure grid coordinates are valid
        if (gridX >= 0 && gridX < CONFIG.gridSize && gridY >= 0 && gridY < CONFIG.gridSize) {
          // Add density at mouse position using the configured amount
          fluidSim.add_density(gridX, gridY, CONFIG.densityAmount);
          
          // Add velocity based on mouse movement with configurable scaling
          const dx = Math.max(-10, Math.min(10, mouseX - lastMouseX));
          const dy = Math.max(-10, Math.min(10, mouseY - lastMouseY));
          fluidSim.add_velocity(gridX, gridY, dx * CONFIG.velocityScale, dy * CONFIG.velocityScale);
        }
        
        lastMouseX = mouseX;
        lastMouseY = mouseY;
      } catch (error) {
        console.error("Error in mouse interaction:", error);
      }
    }
  });

  // Frame counter for frame skipping
  let frameCount = 0;

  // Animation loop
  function animate() {
    try {
      // Clear canvas with configured background color
      const [r, g, b] = CONFIG.backgroundColor;
      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      
      // Only process physics on appropriate frames (allows frame skipping for performance)
      if (frameCount % (CONFIG.frameSkip + 1) === 0) {
        // Apply density and velocity decay if configured
        if (CONFIG.densityDecay !== 1.0 || CONFIG.velocityDecay !== 1.0) {
          fluidSim.apply_decay(CONFIG.densityDecay, CONFIG.velocityDecay);
        }
        
        // Step simulation
        fluidSim.step();
      }
      
      // Render fluid with configured color settings
      fluidSim.render_with_color(
        ctx, 
        canvas, 
        CONFIG.fluidColor[0],
        CONFIG.fluidColor[1], 
        CONFIG.fluidColor[2],
        CONFIG.colorIntensity
      );
      
      // Optionally draw velocity visualization
      if (CONFIG.showVelocity) {
        drawVelocityField(fluidSim, ctx, canvas);
      }
      
      // Increment frame counter
      frameCount = (frameCount + 1) % 1000; // Prevent potential overflow
    } catch (error) {
      console.error("Error in animation loop:", error);
      // Continue the animation loop even if there's an error
    }
    
    requestAnimationFrame(animate);
  }
  
  // Helper function to visualize velocity field
  function drawVelocityField(sim, ctx, canvas) {
    const cellWidth = canvas.width / CONFIG.gridSize;
    const cellHeight = canvas.height / CONFIG.gridSize;
    
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)';
    ctx.lineWidth = 1;
    
    // Draw every 5th vector to avoid clutter
    for (let i = 5; i < CONFIG.gridSize; i += 5) {
      for (let j = 5; j < CONFIG.gridSize; j += 5) {
        const vx = sim.get_velocity_x(i, j);
        const vy = sim.get_velocity_y(i, j);
        
        const x = i * cellWidth;
        const y = j * cellHeight;
        
        ctx.beginPath();
        ctx.moveTo(x, y);
        ctx.lineTo(x + vx * 50, y + vy * 50); // Scale for visibility
        ctx.stroke();
      }
    }
  }

  animate();
});