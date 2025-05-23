# Water Fluid Simulation

A fluid simulation visualization based on the Navier-Stokes equations, implemented in Rust and compiled to WebAssembly.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/)
- [npm](https://www.npmjs.com/)

## Setup

1. Clone this repository
2. Install dependencies:
   ```
   npm install
   ```

## Development

To start the development server:

```
npm start
```

This will compile the Rust code to WASM, bundle it with webpack, and start a development server.

## Usage

- Click and drag on the canvas to add fluid and velocity
- The fluid will flow based on the Navier-Stokes equations
- Experiment with different parameters in the code to get different visual effects

## Building for Production

To build for production:

```
npm run build
```

This will create optimized files in the `dist` directory.

## Adding to a Static Webpage

To add this fluid simulation to any static webpage:

1. Build the project for production:
   ```
   npm run build
   ```

2. Copy the following files from the `dist` directory to your website:
   - `index.html` (as a reference or directly use it)
   - `js/index.js`
   - Any `.wasm` file (usually named with a hash like `27322e9928a5ed56e00d.wasm`)

3. Add the following HTML to your webpage:
   ```html
   <canvas id="fluid-canvas"></canvas>
   <script src="path/to/index.js"></script>
   ```

4. Make sure your canvas has proper sizing with CSS:
   ```css
   #fluid-canvas {
     width: 100%;
     height: 400px;
     background-color: black;
   }
   ```

5. If you want to customize the fluid simulation, you can modify the `CONFIG` object in `src/web/index.js`:

   ```javascript
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
   ```

### Alternative: Using Pre-compiled Files

For simple integration without rebuilding:

1. Use the pre-compiled files in the `dist` directory after running `npm run build`
2. Host these files on any static file server
3. Link to the HTML file directly or embed the canvas and JavaScript in your existing page

### Deploying to GitHub Pages

To deploy the simulation to GitHub Pages:

1. Build the project:
   ```
   npm run build
   ```

2. Create a GitHub repository for your project

3. Configure your `package.json` with a GitHub Pages deploy script:
   ```json
   "scripts": {
     "deploy": "gh-pages -d dist"
   }
   ```

4. Install the gh-pages package:
   ```
   npm install --save-dev gh-pages
   ```

5. Deploy to GitHub Pages:
   ```
   npm run deploy
   ```

6. Access your fluid simulation at `https://your-username.github.io/your-repo-name`

## How It Works

This simulation implements the Navier-Stokes equations for incompressible fluids in 2D. The implementation is based on Jos Stam's "Real-Time Fluid Dynamics for Games" paper (2003).

The fluid simulation consists of several key steps:
1. **Diffusion**: Viscosity causes the fluid to spread out
2. **Advection**: The fluid moves along the velocity field
3. **Projection**: Enforce mass conservation (incompressibility)

## License

MIT