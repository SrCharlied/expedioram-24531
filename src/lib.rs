//! Renderer del diorama *El Continente Inacabado*.
//!
//! Aquí vive todo lo que se puede probar sin abrir una ventana: geometría,
//! intersecciones, color y el trazado en sí. El binario se queda solo con
//! lo que no se puede probar de esa forma —crear la ventana, leer el
//! teclado y presentar el framebuffer—, así que la lógica del raytracer
//! queda accesible desde `cargo test` y desde un render sin ventana.

pub mod camera;
pub mod color;
pub mod framebuffer;
pub mod ray_intersect;
pub mod renderer;
pub mod sphere;
