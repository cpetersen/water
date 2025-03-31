use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct FluidSimulation {
    width: usize,
    height: usize,
    density: Vec<f64>,
    velocity_x: Vec<f64>,
    velocity_y: Vec<f64>,
    diff: f64,
    visc: f64,
    dt: f64,
}

#[wasm_bindgen]
impl FluidSimulation {
    pub fn new(width: usize, height: usize, diff: f64, visc: f64, dt: f64) -> FluidSimulation {
        let size = width * height;
        
        // Set up the panic hook if in debug build
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        FluidSimulation {
            width,
            height,
            density: vec![0.0; size],
            velocity_x: vec![0.0; size],
            velocity_y: vec![0.0; size],
            diff,
            visc,
            dt,
        }
    }

    pub fn step(&mut self) {
        self.velocity_step();
        self.density_step();
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f64) {
        let idx = self.get_index(x, y);
        self.density[idx] += amount;
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f64, amount_y: f64) {
        let idx = self.get_index(x, y);
        self.velocity_x[idx] += amount_x;
        self.velocity_y[idx] += amount_y;
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement) {
        let width = canvas.width() as usize;
        let height = canvas.height() as usize;
        
        for i in 0..self.width {
            for j in 0..self.height {
                let idx = self.get_index(i, j);
                let density = self.density[idx];
                
                let color = format!("rgba(0, 0, 255, {})", density);
                // Using the deprecated method for now since the alternative isn't working
                ctx.set_fill_style(&JsValue::from_str(&color));
                
                let cell_width = width as f64 / self.width as f64;
                let cell_height = height as f64 / self.height as f64;
                
                ctx.fill_rect(
                    i as f64 * cell_width,
                    j as f64 * cell_height,
                    cell_width,
                    cell_height,
                );
            }
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn velocity_step(&mut self) {
        // Create temporary vectors for diffusion step
        let size = self.width * self.height;
        let mut velocity_x0 = vec![0.0; size];
        let mut velocity_y0 = vec![0.0; size];
        
        // Diffuse velocities
        self.diffuse(1, &mut velocity_x0, &self.velocity_x, self.visc);
        self.diffuse(2, &mut velocity_y0, &self.velocity_y, self.visc);
        
        // Project to maintain mass conservation
        self.project(&mut velocity_x0, &mut velocity_y0);
        
        // Move values from temp vectors back to main vectors for advection
        std::mem::swap(&mut self.velocity_x, &mut velocity_x0);
        std::mem::swap(&mut self.velocity_y, &mut velocity_y0);
        
        // Advect velocities
        self.advect(1, &mut velocity_x0, &self.velocity_x, &self.velocity_x, &self.velocity_y);
        self.advect(2, &mut velocity_y0, &self.velocity_y, &self.velocity_x, &self.velocity_y);
        
        // Move advected values back to main vectors
        std::mem::swap(&mut self.velocity_x, &mut velocity_x0);
        std::mem::swap(&mut self.velocity_y, &mut velocity_y0);
        
        // Project again using copies of the vectors to avoid borrow checker issues
        let vel_x = self.velocity_x.clone();
        let vel_y = self.velocity_y.clone();
        self.project_from_copies(&vel_x, &vel_y);
    }

    fn project(&mut self, velocity_x: &mut Vec<f64>, velocity_y: &mut Vec<f64>) {
        let n = self.width as f64;
        let size = self.width * self.height;
        let mut div = vec![0.0; size];
        let mut p = vec![0.0; size];
        
        // Calculate divergence
        for i in 1..self.width-1 {
            for j in 1..self.height-1 {
                let idx = self.get_index(i, j);
                let idx_right = self.get_index(i+1, j);
                let idx_left = self.get_index(i-1, j);
                let idx_bottom = self.get_index(i, j+1);
                let idx_top = self.get_index(i, j-1);
                
                div[idx] = -0.5 * (
                    velocity_x[idx_right] - velocity_x[idx_left] +
                    velocity_y[idx_bottom] - velocity_y[idx_top]
                ) / n;
                
                p[idx] = 0.0;
            }
        }
        
        self.set_boundaries(0, &mut div);
        self.set_boundaries(0, &mut p);
        
        // Solve pressure
        self.lin_solve(0, &mut p, &div, 1.0, 4.0);
        
        // Subtract pressure gradient from velocity
        for i in 1..self.width-1 {
            for j in 1..self.height-1 {
                let idx = self.get_index(i, j);
                let idx_right = self.get_index(i+1, j);
                let idx_left = self.get_index(i-1, j);
                let idx_bottom = self.get_index(i, j+1);
                let idx_top = self.get_index(i, j-1);
                
                velocity_x[idx] -= 0.5 * (p[idx_right] - p[idx_left]) * n;
                velocity_y[idx] -= 0.5 * (p[idx_bottom] - p[idx_top]) * n;
            }
        }
        
        self.set_boundaries(1, velocity_x);
        self.set_boundaries(2, velocity_y);
    }
    
    // Special version of project that updates self.velocity_x and self.velocity_y directly
    fn project_from_copies(&mut self, velocity_x: &Vec<f64>, velocity_y: &Vec<f64>) {
        let n = self.width as f64;
        let size = self.width * self.height;
        let mut div = vec![0.0; size];
        let mut p = vec![0.0; size];
        
        // Calculate divergence
        for i in 1..self.width-1 {
            for j in 1..self.height-1 {
                let idx = self.get_index(i, j);
                let idx_right = self.get_index(i+1, j);
                let idx_left = self.get_index(i-1, j);
                let idx_bottom = self.get_index(i, j+1);
                let idx_top = self.get_index(i, j-1);
                
                div[idx] = -0.5 * (
                    velocity_x[idx_right] - velocity_x[idx_left] +
                    velocity_y[idx_bottom] - velocity_y[idx_top]
                ) / n;
                
                p[idx] = 0.0;
            }
        }
        
        self.set_boundaries(0, &mut div);
        self.set_boundaries(0, &mut p);
        
        // Solve pressure
        self.lin_solve(0, &mut p, &div, 1.0, 4.0);
        
        // Create temp vectors to store the updated velocities
        let mut new_velocity_x = self.velocity_x.clone();
        let mut new_velocity_y = self.velocity_y.clone();
        
        // Subtract pressure gradient from velocity
        for i in 1..self.width-1 {
            for j in 1..self.height-1 {
                let idx = self.get_index(i, j);
                let idx_right = self.get_index(i+1, j);
                let idx_left = self.get_index(i-1, j);
                let idx_bottom = self.get_index(i, j+1);
                let idx_top = self.get_index(i, j-1);
                
                new_velocity_x[idx] -= 0.5 * (p[idx_right] - p[idx_left]) * n;
                new_velocity_y[idx] -= 0.5 * (p[idx_bottom] - p[idx_top]) * n;
            }
        }
        
        // Set boundaries on the temp vectors
        self.set_boundaries(1, &mut new_velocity_x);
        self.set_boundaries(2, &mut new_velocity_y);
        
        // Replace the original vectors with the updated ones
        self.velocity_x = new_velocity_x;
        self.velocity_y = new_velocity_y;
    }

    fn density_step(&mut self) {
        let size = self.width * self.height;
        let mut density0 = vec![0.0; size];
        
        // Diffuse density
        self.diffuse(0, &mut density0, &self.density, self.diff);
        
        // Move density from temp vector to main vector
        std::mem::swap(&mut self.density, &mut density0);
        
        // Advect density
        self.advect(0, &mut density0, &self.density, &self.velocity_x, &self.velocity_y);
        
        // Move advected density back to main vector
        std::mem::swap(&mut self.density, &mut density0);
    }

    fn diffuse(&self, b: usize, x: &mut Vec<f64>, x0: &Vec<f64>, diff: f64) {
        let a = self.dt * diff * (self.width - 2) as f64 * (self.height - 2) as f64;
        self.lin_solve(b, x, x0, a, 1.0 + 4.0 * a);
    }

    fn lin_solve(&self, b: usize, x: &mut Vec<f64>, x0: &Vec<f64>, a: f64, c: f64) {
        let iter = 4; // Number of Gauss-Seidel relaxation iterations
        
        for _ in 0..iter {
            for i in 1..self.width-1 {
                for j in 1..self.height-1 {
                    let idx = self.get_index(i, j);
                    let idx_left = self.get_index(i-1, j);
                    let idx_right = self.get_index(i+1, j);
                    let idx_top = self.get_index(i, j-1);
                    let idx_bottom = self.get_index(i, j+1);
                    
                    x[idx] = (x0[idx] + a * (
                            x[idx_left] + x[idx_right] +
                            x[idx_top] + x[idx_bottom]
                        )) / c;
                }
            }
            
            self.set_boundaries(b, x);
        }
    }

    fn advect(&self, b: usize, d: &mut Vec<f64>, d0: &Vec<f64>, u: &Vec<f64>, v: &Vec<f64>) {
        let dt0 = self.dt * (self.width - 2) as f64;
        
        for i in 1..self.width-1 {
            for j in 1..self.height-1 {
                let idx = self.get_index(i, j);
                
                // Trace particle back
                let mut x = i as f64 - dt0 * u[idx];
                let mut y = j as f64 - dt0 * v[idx];
                
                // Ensure in bounds
                x = x.max(0.5).min((self.width - 1) as f64 - 0.5);
                y = y.max(0.5).min((self.height - 1) as f64 - 0.5);
                
                // Bilinear interpolation
                let i0 = x.floor() as usize;
                let i1 = i0 + 1;
                let j0 = y.floor() as usize;
                let j1 = j0 + 1;
                
                let s1 = x - i0 as f64;
                let s0 = 1.0 - s1;
                let t1 = y - j0 as f64;
                let t0 = 1.0 - t1;
                
                let idx00 = self.get_index(i0, j0);
                let idx10 = self.get_index(i1, j0);
                let idx01 = self.get_index(i0, j1);
                let idx11 = self.get_index(i1, j1);
                
                d[idx] = s0 * (t0 * d0[idx00] + t1 * d0[idx01]) +
                         s1 * (t0 * d0[idx10] + t1 * d0[idx11]);
            }
        }
        
        self.set_boundaries(b, d);
    }

    fn set_boundaries(&self, b: usize, x: &mut Vec<f64>) {
        // Set boundary conditions
        let w = self.width;
        let h = self.height;
        
        // Left/right walls
        for j in 1..h-1 {
            let idx_left = self.get_index(0, j);
            let idx_right = self.get_index(w-1, j);
            let idx_next_left = self.get_index(1, j);
            let idx_next_right = self.get_index(w-2, j);
            
            x[idx_left] = if b == 1 { -x[idx_next_left] } else { x[idx_next_left] };
            x[idx_right] = if b == 1 { -x[idx_next_right] } else { x[idx_next_right] };
        }
        
        // Top/bottom walls
        for i in 1..w-1 {
            let idx_top = self.get_index(i, 0);
            let idx_bottom = self.get_index(i, h-1);
            let idx_next_top = self.get_index(i, 1);
            let idx_next_bottom = self.get_index(i, h-2);
            
            x[idx_top] = if b == 2 { -x[idx_next_top] } else { x[idx_next_top] };
            x[idx_bottom] = if b == 2 { -x[idx_next_bottom] } else { x[idx_next_bottom] };
        }
        
        // Corners
        let idx_top_left = self.get_index(0, 0);
        let idx_top_right = self.get_index(w-1, 0);
        let idx_bottom_left = self.get_index(0, h-1);
        let idx_bottom_right = self.get_index(w-1, h-1);
        
        x[idx_top_left] = 0.5 * (x[self.get_index(1, 0)] + x[self.get_index(0, 1)]);
        x[idx_top_right] = 0.5 * (x[self.get_index(w-2, 0)] + x[self.get_index(w-1, 1)]);
        x[idx_bottom_left] = 0.5 * (x[self.get_index(1, h-1)] + x[self.get_index(0, h-2)]);
        x[idx_bottom_right] = 0.5 * (x[self.get_index(w-2, h-1)] + x[self.get_index(w-1, h-2)]);
    }
}