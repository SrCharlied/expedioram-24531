//! Renderer del diorama *El Continente Inacabado*.
//!
//! Aquí vive todo lo que se puede probar sin abrir una ventana: geometría,
//! intersecciones, color y el trazado en sí. El binario se queda solo con
//! lo que no se puede probar de esa forma —crear la ventana, leer el
//! teclado y presentar el framebuffer—, así que la lógica del raytracer
//! queda accesible desde `cargo test` y desde un render sin ventana.

pub mod accel;
pub mod bounds;
pub mod camera;
pub mod color;
pub mod cuboid;
pub mod framebuffer;
pub mod hit;
pub mod light;
pub mod material;
pub mod optics;
pub mod primitive;
pub mod ray;
pub mod ray_intersect;
pub mod renderer;
pub mod reveal;
pub mod scene;
pub mod scene_builder;
pub mod scenes;
pub mod skybox;
pub mod texture;

/// Margen para despegar un rayo secundario de la superficie que lo originó.
///
/// Sin él, el rayo vuelve a impactar el mismo punto del que sale por error
/// de redondeo: es el acné de sombras. Un solo valor canónico para todo el
/// proyecto —intersecciones, rayos de sombra, reflexión y refracción— para
/// que no aparezcan tres epsilons distintos que haya que reconciliar.
pub const EPSILON: f32 = 1e-4;
