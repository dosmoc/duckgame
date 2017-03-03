extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use graphics::color as color;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


#[derive(Debug, Copy, Clone)]
enum Color {
    BROWN,
	GREEN,
	BLUE,
	RED,
	TEAL
}

fn get_color(the_color: Color) -> [f32; 4] {
  	match the_color {
        Color::BROWN => color::hex("b16f30"),
  		Color::GREEN => [0.0, 1.0, 0.0, 1.0],
  		Color::BLUE  => color::hex("6f9ae8"),
  		Color::RED   => [1.0, 0.0, 0.0, 1.0],
  		Color::TEAL   => [0.5, 0.8, 0.8, 1.0]
  	}
}

fn window_dimensions(w: &Window) -> (f64, f64) {
  let window_size = match w.window.get_inner_size_points() {
    Some(ws) => (ws.0 as f64, ws.1 as f64),
    None => (0.0, 0.0)
  };

  window_size
}


pub struct Duck {
    rotation: f64, //Rotation for the square
    rotation_rate: f64,
    x: f64, //x coordinate of square
    y: f64, //y coordinate of square
    the_color: Color,
    acceleration: f64, //does not change over time for now
    graphics: [[f64; 2]; 4], //just a triangle for now
    moving_forward: bool, 
    moving_backward: bool, 
    rotating_left: bool, 
    rotating_right: bool
}

pub struct Game {
    gl: GlGraphics, //OpenGl drawing backend
    momma_duck: Duck,
    /*
    rotation: f64, //Rotation for the square
    x: f64, //x coordinate of square
    y: f64, //y coordinate of square
    square_radius: f64,
    the_color: Color,
    speed: f64 */
}

impl Duck {

  fn render(&mut self, c: graphics::Context, gl: &mut GlGraphics) {
    use graphics::{ Transformed, polygon };
    let rotation = self.rotation;
    let (x, y) = (self.x, self.y);
    let transform = c.transform.trans(x, y).rot_rad(rotation);

    //polygon draws a shape between the points of the third parameter
    polygon(get_color(self.the_color), &self.graphics, transform, gl);

  } 

  fn update(&mut self, window: &Window, args: &UpdateArgs) {
//    self.rotation += 2.0 * args.dt;

    let window_size = window_dimensions(window);
    let (window_width, window_height) = window_size;
    
    //wrap the if it goes off edges
    if self.x > window_width {
      self.x = 0.0;
    } else if self.x < 0.0 {
      self.x = window_width;
    }

    if self.y > window_height {
      self.y = 0.0;
    } else if self.y < 0.0 {
      self.y = window_height;
    }

    //rotate base on input
    if self.rotating_left {
      self.rotation -= self.rotation_rate * args.dt;    
    }
    if self.rotating_right {
      self.rotation += self.rotation_rate * args.dt;    
    }
    
    
    if self.moving_forward {
      self.x += self.acceleration * self.rotation.cos() * args.dt;
      self.y += self.acceleration * self.rotation.sin() * args.dt;
    } 

    if self.moving_backward {
      self.x -= self.acceleration * self.rotation.cos() * args.dt;
      self.y -= self.acceleration * self.rotation.sin() * args.dt;
    } 
  }


  fn key_press(&mut self, key: Key) {
    match key {
       Key::Up => self.moving_forward = true,
       Key::Down => self.moving_backward = true,
       Key::Left => self.rotating_left = true,
       Key::Right => self.rotating_right = true,
      _ => {}
    }
  }

  fn key_release(&mut self, key: Key) {
    match key {
       Key::Up => self.moving_forward = false,
       Key::Down => self.moving_backward = false,
       Key::Left => self.rotating_left = false,
       Key::Right => self.rotating_right = false,
      _ => {}
    }

    match (self.the_color, key) {
      (Color::RED, Key::Space) =>
            self.the_color = Color::GREEN,
      (Color::GREEN, Key::Space) => 
            self.the_color = Color::RED,
      _ => {}
    }
  }

}

impl Game {
  fn render(&mut self, args: &RenderArgs) {
      use graphics::*;
      
      let momma_duck = &mut self.momma_duck;

      self.gl.draw(args.viewport(), |c, gl| {
          //Clear the screen
          clear(get_color(Color::BLUE), gl);

          momma_duck.render(c, gl);
      });
  }

  fn update(&mut self, window: &Window, args: &UpdateArgs) {
    // Rotate 2 radians per second
    let momma_duck = &mut self.momma_duck;    

    momma_duck.update(window, args);
  }

  fn key_release(&mut self, key: Key) {
    let momma_duck = &mut self.momma_duck;    

    momma_duck.key_release(key);
  }


  fn key_press(&mut self, key: Key) {
    let momma_duck = &mut self.momma_duck;    

    momma_duck.key_press(key);
  }

}

fn main() {
  let opengl = OpenGL::V3_2;
  let mut window: Window = WindowSettings::new(
         "spinning-duck",
         [200, 200]
      )
      .opengl(opengl)
      .exit_on_esc(true)
   
      .build()
      .unwrap();
 
  //we get the initial window size
  let window_size = window_dimensions(&window);
  //then get the center so the square
  //will start off at the center
  let (x, y) = ((window_size.0 / 2.0),
                (window_size.1 / 2.0));

  let momma = Duck {
    rotation: 0.0,
    rotation_rate: 2.0,
    x: x,
    y: y,
    the_color: Color::BROWN,
    acceleration: 70.0,
    //These are points of a polygon
    graphics: [[10.0, 10.0], [-10.0, 10.0,], [-10.0, -10.0], [10.0, -10.0]],
    moving_forward: false, 
    moving_backward: false, 
    rotating_left: false, 
    rotating_right: false
  };

  let mut app = Game {
    gl: GlGraphics::new(opengl),
    momma_duck: momma
  };

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
        app.render(&r);
    }

    if let Some(u) = e.update_args() {
        app.update(&window, &u);
    }
    
    if let Some(Button::Keyboard(key)) = e.press_args() {
        app.key_press(key);
    }

    if let Some(Button::Keyboard(key)) = e.release_args() {
    	app.key_release(key)
    }
  }
}
