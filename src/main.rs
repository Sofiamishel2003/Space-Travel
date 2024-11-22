use nalgebra_glm::{Vec3, Vec4, Mat4, look_at, perspective, ortho};
use minifb::{Key, Window, WindowOptions};
use std::{f32::consts::PI, time::Instant};

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod skybox;
mod frustum;

use frustum::{Frustum, Plane};
use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use skybox::Skybox; 
use triangle::triangle;
use shaders::{vertex_shader,sun_shader, fragment_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};
use color::Color;
// Estructuras--------------------------------------------------------------
pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
    cloud_noise: FastNoiseLite, 
    current_shader: u8, 
}
struct AppState {
    last_mouse_pos: Option<(f32, f32)>,
    bird_eye_active: bool,
}
// Noises ---------------------------------------------------------------------------------------------------------
fn create_noise(current_shader: u8) -> FastNoiseLite {
    match current_shader  {
        1 => create_sun_noise(),
        2 => create_mars_noise(),
        3 => create_earth_noise(),
        4 => create_jupiter_noise(),
        5 => create_mercury_noise(),
        6 => create_uranus_noise(),
        7 => create_saturn_noise(),
        8 => create_moon_noise(),
        9 => FastNoiseLite::new(),
        10 => create_spaceship_noise(),
        _ => FastNoiseLite::new(),
    }
}
fn create_sun_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    
    // Use FBm for multi-layered noise, giving a "turbulent" feel
    noise.set_noise_type(Some(NoiseType::Perlin));  // Perlin noise for smooth, natural texture
    noise.set_fractal_type(Some(FractalType::FBm)); // FBm for layered detail
    noise.set_fractal_octaves(Some(10));             // High octaves for rich detail
    noise.set_fractal_lacunarity(Some(2.0));        // Higher lacunarity = more contrast between layers
    noise.set_fractal_gain(Some(0.5));              // Higher gain = more influence of smaller details
    noise.set_frequency(Some(0.02));                // Low frequency = large features
    
    noise
}
fn create_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(5)); // Octavas para mayor detalle
    noise.set_fractal_lacunarity(Some(3.0)); // Lacunaridad para escalado de frecuencia
    noise.set_fractal_gain(Some(0.5)); // Ganancia para el escalado de amplitud
    noise.set_frequency(Some(0.5)); 
    noise
}
fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1234); // Use a unique seed for clouds
    noise.set_noise_type(Some(NoiseType::Perlin));  // Smooth noise for clouds
    noise.set_fractal_type(Some(FractalType::FBm)); // Fractal noise for depth
    noise.set_fractal_octaves(Some(6));             // Higher octaves for more detail
    noise.set_fractal_lacunarity(Some(2.0));        // Higher lacunarity for contrast
    noise.set_fractal_gain(Some(0.5));              // Balance smaller and larger features
    noise.set_frequency(Some(0.01));               // Low frequency for large cloud structures
    noise
}
fn create_mars_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    
    // Usamos Perlin para obtener una textura suave para Marte
    noise.set_noise_type(Some(NoiseType::Perlin)); // Ruido de Perlin para suavidad
    noise.set_fractal_type(Some(FractalType::FBm)); // Fractal FBm para generar texturas más naturales
    noise.set_fractal_octaves(Some(5));             // Establecemos el número de octavas para mayor detalle
    noise.set_fractal_lacunarity(Some(2.0));        // Lacunaridad para mayor contraste entre capas
    noise.set_fractal_gain(Some(0.5));              // Ajusta la ganancia para dar más variación a las capas más finas
    noise.set_frequency(Some(0.01));                // Baja frecuencia para características grandes de la superficie de Marte
    
    noise
}
fn create_jupiter_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    noise.set_noise_type(Some(NoiseType::Perlin));  // Perlin for banded structure
    noise.set_fractal_type(Some(FractalType::FBm)); // Add depth to bands
    noise.set_fractal_octaves(Some(6));             // Detailed turbulence
    noise.set_fractal_lacunarity(Some(1.8));        // Emphasize band transitions
    noise.set_fractal_gain(Some(0.45));             // Enhance smaller turbulence
    noise.set_frequency(Some(0.02));               // Scale of gas bands
    
    noise
}
fn create_mercury_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42); // Usamos un semilla específica para Mercurio
    noise.set_noise_type(Some(NoiseType::Perlin));   // Ruido de Perlin para superficie
    noise.set_fractal_type(Some(FractalType::FBm));  // Textura fractal para detalles
    noise.set_fractal_octaves(Some(10));              // Detalles más finos
    noise.set_fractal_lacunarity(Some(2.5));         // Contraste en las capas de ruido
    noise.set_fractal_gain(Some(0.5));               // Ajuste de la influencia del ruido
    noise.set_frequency(Some(0.005));                // Baja frecuencia para grandes características
    
    noise
}


fn create_uranus_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    noise.set_noise_type(Some(NoiseType::Perlin));  // Replace Simplex with Perlin
    noise.set_fractal_type(Some(FractalType::FBm)); 
    noise.set_fractal_octaves(Some(4));              
    noise.set_fractal_lacunarity(Some(2.2));         
    noise.set_fractal_gain(Some(0.5));               
    noise.set_frequency(Some(0.008));               
    
    noise
}

fn create_saturn_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    noise.set_noise_type(Some(NoiseType::Perlin));  // Perlin for banded structure
    noise.set_fractal_type(Some(FractalType::FBm)); // Add depth to bands
    noise.set_fractal_octaves(Some(6));             // Detailed turbulence
    noise.set_fractal_lacunarity(Some(1.8));        // Emphasize band transitions
    noise.set_fractal_gain(Some(0.45));             // Enhance smaller turbulence
    noise.set_frequency(Some(0.02));               // Scale of gas bands
    
    noise
}
fn create_moon_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(4321);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(2));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(3.0));  
    
    noise
}
fn create_spaceship_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(2021);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(4));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.4));
    noise.set_frequency(Some(0.5));
    
    noise
}
// VIEW ---------------------------------------------------------------------------------------------------------
fn check_collision(camera_pos: Vec3, planet_pos: Vec3, planet_radius: f32) -> bool {
    let distance = (camera_pos - planet_pos).magnitude();
    distance < (planet_radius + 25.0)  // margen de seguridad
}
fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}
// RENDERS  ---------------------------------------------------------------------------------------------------------
fn render_orbit_lines(
    framebuffer: &mut Framebuffer,
    orbit_radius: f32,
    color: Color,
    segments: usize,
    uniforms: &Uniforms,
) {
    framebuffer.set_current_color(color.to_hex());

    let orbit_depth = 0.95; // Profundidad para las órbitas, más lejos que los planetas

    for i in 0..segments {
        let angle1 = 2.0 * PI * (i as f32) / (segments as f32);
        let angle2 = 2.0 * PI * ((i + 1) as f32) / (segments as f32);

        let world_pos1 = Vec4::new(
            orbit_radius * angle1.cos(),
            0.0,
            orbit_radius * angle1.sin(),
            1.0,
        );
        let world_pos2 = Vec4::new(
            orbit_radius * angle2.cos(),
            0.0,
            orbit_radius * angle2.sin(),
            1.0,
        );

        let clip_pos1 = uniforms.projection_matrix * uniforms.view_matrix * world_pos1;
        let clip_pos2 = uniforms.projection_matrix * uniforms.view_matrix * world_pos2;

        let ndc_pos1 = Vec3::new(
            clip_pos1.x / clip_pos1.w,
            clip_pos1.y / clip_pos1.w,
            orbit_depth,
        );
        let ndc_pos2 = Vec3::new(
            clip_pos2.x / clip_pos2.w,
            clip_pos2.y / clip_pos2.w,
            orbit_depth,
        );

        let screen_pos1 =
            uniforms.viewport_matrix * Vec4::new(ndc_pos1.x, ndc_pos1.y, ndc_pos1.z, 1.0);
        let screen_pos2 =
            uniforms.viewport_matrix * Vec4::new(ndc_pos2.x, ndc_pos2.y, ndc_pos2.z, 1.0);

        let screen_x1 = screen_pos1.x as usize;
        let screen_y1 = screen_pos1.y as usize;
        let screen_x2 = screen_pos2.x as usize;
        let screen_y2 = screen_pos2.y as usize;

        if screen_x1 < framebuffer.width
            && screen_y1 < framebuffer.height
            && screen_x2 < framebuffer.width
            && screen_y2 < framebuffer.height
        {
            framebuffer.line_with_depth(screen_x1, screen_y1, screen_x2, screen_y2, orbit_depth);
        }
    }
}



fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], time: u32, disable_culling: bool) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], disable_culling));
    }
    let emission = 0; 
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = fragment_shader(&fragment, &uniforms);
            framebuffer.set_current_color(color.to_hex());
            framebuffer.point(x, y, fragment.depth, emission); 
        }
    }
}
fn create_ortho_projection(window_width: f32, window_height: f32) -> Mat4 {
    nalgebra_glm::ortho(0.0, window_width, window_height, 0.0, -1.0, 1.0)
}

fn main() {
    let window_width = 800;
    let window_height = 700;
    let framebuffer_width = 800;
    let framebuffer_height = 700;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Planets Shaders",
        window_width,
        window_height,
        WindowOptions::default(),
    )
        .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000);

    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 2.0f32;

    let mut camera = Camera::new(
        Vec3::new(0.0, 80.0, -200.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );


    let mut app_state = AppState {
        last_mouse_pos: None,
        bird_eye_active: false,
    };
    let obj = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let moon = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let ring_obj = Obj::load("assets/ring.obj").expect("Failed to load ring model");
    let spaceship = Obj::load("assets/spaceShip1.obj").expect("Failed to load spaceship model");
    let skybox = Skybox::new(1000);

    let vertex_arrays = obj.get_vertex_array(); 
    let moon_vertex_array = moon.get_vertex_array();
    let ring_vertex_array = ring_obj.get_vertex_array();
    let spaceship_vertex_array = spaceship.get_vertex_array();

    let mut last_frame_time = Instant::now();
    let mut time = 0;

    // Lunas de los planetas rocosos
    let moon_scale = 1.8; 
    let moon_distance = 2.8;
    let moon_orbit_speed = 0.001;

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    let mut uniforms = Uniforms { 
        model_matrix: Mat4::identity(), 
        view_matrix: Mat4::identity(), 
        projection_matrix, 
        viewport_matrix, 
        time: 0, 
        noise: create_noise(1),
        cloud_noise: create_cloud_noise(),
        current_shader: 1,
    };

    let planet_positions = vec![
        Vec3::new(0.0, 0.0, 0.0),   // Sol
        Vec3::new(10.0, 2.0, -3.0),  // Mercurio
        Vec3::new(15.0, 0.0, 5.0),  // Tierra
        Vec3::new(25.0, -2.0, -7.0), // Marte
        Vec3::new(35.0, 1.0, 10.0), // Júpiter
        Vec3::new(50.0, 3.0, -5.0), // Urano
        Vec3::new(65.0, -1.0, 7.0), // Saturno
    ];
    let planet_scales = vec![
        14.0,  // Sol
        3.0,   // Mercurio
        5.0,   // Tierra
        7.0,   // Marte
        8.0,  // Júpiter
        10.0,   // Urano
        12.0,   // Saturno
    ];
        
    let orbital_speeds = vec![
        1.0,     // Sol no orbita
        0.9,    // Mercurio
        0.8,    // Tierra
        0.7,   // Marte
        0.6,   // Júpiter
        0.5,   // Urano
        0.4,   // Saturno
    ];
        

    let system_center = Vec3::new(0.0, 0.0, 0.0);

    let shaders = vec![1, 2, 3, 4, 5, 6, 7];
    let mut orbital_angles = vec![0.0; shaders.len()]; 

    let mut bird_eye_active = false; 
    let hud_camera_projection = create_ortho_projection(window_width as f32, window_height as f32);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        let delta_time = last_frame_time.elapsed();
        last_frame_time = Instant::now();
        time += delta_time.as_millis() as u32;
        
        handle_input(&window, &mut camera, system_center, &mut bird_eye_active, &mut app_state);        framebuffer.clear();
        framebuffer.clear();

        uniforms.projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let vp_matrix = uniforms.projection_matrix * uniforms.view_matrix;
        let frustum = Frustum::from_matrix(&vp_matrix);


        skybox.render(&mut framebuffer, &uniforms, camera.eye);

        // Verifica las colisiones antes de actualizar la posición de la cámara y renderizar los objetos
        for (planet_pos, planet_radius) in planet_positions.iter().zip(planet_scales.iter()) {
            if check_collision(camera.eye, *planet_pos, *planet_radius) {
                camera.prevent_collision(*planet_pos, *planet_radius);
            }
        }
        // Matriz de visión siempre actualizada
        uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        uniforms.time = time as u32;
        for (i, o_pos) in planet_positions.iter().enumerate() {
            let orbit_radius = o_pos.x;
            orbital_angles[i] += orbital_speeds[i] * delta_time.as_secs_f32();
        
            // Renderizar órbitas (con profundidad lejana)
            let orbit_color = Color::new(100, 100, 100); // Gris más oscuro
            render_orbit_lines(&mut framebuffer, orbit_radius, orbit_color, 100, &uniforms);
        }        
        for (i, o_pos) in planet_positions.iter().enumerate() {
            let orbit_radius = o_pos.x;  
            orbital_angles[i] += orbital_speeds[i] * delta_time.as_secs_f32(); // Asumiendo que orbital_speeds está definido
            let planet_x = orbit_radius * orbital_angles[i].cos();
            let planet_z = orbit_radius * orbital_angles[i].sin();
            let position = Vec3::new(planet_x, 0.0, planet_z); // Asumiendo una órbita plana en el plano xz
            let scale = planet_scales[i]; 
            if frustum.contains(position, scale) {
                uniforms.current_shader = shaders[i];
                uniforms.model_matrix = create_model_matrix(position, scale, rotation);
                render(&mut framebuffer, &uniforms, &vertex_arrays, time as u32, false);
                if shaders[i] == 3 {  
                    let moon_angle = time as f32 * moon_orbit_speed;
                        let moon_x = moon_distance * moon_angle.cos();
                        let moon_z = moon_distance * moon_angle.sin();
                        let moon_translation = Vec3::new(moon_x, 0.0, moon_z) + position;

                        if frustum.contains(moon_translation, moon_scale) {
                            uniforms.current_shader = 8;
                            uniforms.model_matrix = create_model_matrix(moon_translation, moon_scale, Vec3::new(0.0, 0.0, 0.0));
                            render(&mut framebuffer, &uniforms, &moon_vertex_array, time as u32, false);
                        }
                } 
                else if shaders[i] == 7 {  
                    let ring_scale = scale * 0.30;
                    let ring_position = position;  
                    uniforms.current_shader = 9;
                    uniforms.model_matrix = create_model_matrix(ring_position, ring_scale, Vec3::new(0.0, 0.0, 0.0));
                    render(&mut framebuffer, &uniforms, &ring_vertex_array, time as u32, true);
                } 
            }
        }
        uniforms.projection_matrix = hud_camera_projection;
        uniforms.view_matrix = Mat4::identity();
        uniforms.model_matrix = create_model_matrix(Vec3::new(window_width as f32 / 2.0, window_height as f32 - 100.0, 0.0), 15.0, Vec3::new(PI, 0.0, 0.0));

        uniforms.current_shader = 10;
        render(&mut framebuffer, &uniforms, &spaceship_vertex_array, time as u32, false);

    
        framebuffer.set_current_color(0xFFDDDD);
        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
    }
}

fn handle_input(window: &Window, camera: &mut Camera, system_center: Vec3, bird_eye_active: &mut bool, app_state: &mut AppState) {
    let movement_speed = 1.0;
    let rotation_speed = PI/25.0;
    let zoom_speed = 20.5;
    let min_zoom_distance = 100.0; 


    let scroll = window.get_scroll_wheel();
    if let Some((_, scroll_y)) = scroll {
        let delta = scroll_y as f32 * zoom_speed;
        camera.zoom(delta, min_zoom_distance);
    }

    //  camera orbit controls
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed, min_zoom_distance);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed, min_zoom_distance);
    }

    if window.is_key_pressed(Key::B, minifb::KeyRepeat::No) {
        if *bird_eye_active {
            camera.eye = Vec3::new(0.0, 80.0, -200.0);
            camera.center = Vec3::new(0.0, 0.0, 0.0);
            camera.up = Vec3::new(0.0, 1.0, 0.0);
            *bird_eye_active = false;
        } else {
            camera.bird_eye_view(system_center, 150.0);
            *bird_eye_active = true;
        }
        camera.has_changed = true;
    }
}