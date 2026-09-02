//! Sonda de poda: compara pruebas de primitiva con y sin jerarquia sobre el
//! blockout real, a resolucion reducida. No es un benchmark de tiempo.
use expedition33_continente_inacabado::accel::TraversalStats;
use expedition33_continente_inacabado::scenes::continent::blockout;

fn main() {
    let diorama = blockout();
    let camera = diorama.hero_camera();
    let scene = diorama.scene;
    let accel = diorama.accel;

    let (ancho, alto) = (160usize, 120usize);
    let mut stats = TraversalStats::default();
    let mut impactos = 0usize;

    for y in 0..alto {
        for x in 0..ancho {
            let ray = camera.ray_from_pixel(x, y, ancho, alto);
            if accel.intersect(&scene, &ray, &mut stats).is_some() {
                impactos += 1;
            }
        }
    }

    let rayos = ancho * alto;
    let sin_accel = rayos * scene.objects.len();

    println!("  rayos                {rayos}");
    println!("  primitivas           {}", scene.objects.len());
    println!(
        "  grupos / clusters    {} / {}",
        accel.groups.len(),
        accel.groups.iter().map(|g| g.clusters.len()).sum::<usize>()
    );
    println!("  impactos             {impactos}");
    println!("  pruebas sin accel    {sin_accel}");
    println!("  pruebas con accel    {}", stats.primitive_tests);
    println!(
        "  reduccion            {:.1}%",
        100.0 * (1.0 - stats.primitive_tests as f64 / sin_accel as f64)
    );
    println!(
        "  bounds grupo/cluster {} / {}",
        stats.group_bounds_tests, stats.cluster_bounds_tests
    );
}
