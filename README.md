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

## How It Works

This simulation implements the Navier-Stokes equations for incompressible fluids in 2D. The implementation is based on Jos Stam's "Real-Time Fluid Dynamics for Games" paper (2003).

The fluid simulation consists of several key steps:
1. **Diffusion**: Viscosity causes the fluid to spread out
2. **Advection**: The fluid moves along the velocity field
3. **Projection**: Enforce mass conservation (incompressibility)

## License

MIT