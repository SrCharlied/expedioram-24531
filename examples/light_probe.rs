//! Imprime las tres luces ya colocadas contra la escala medida del blockout.
use expedition33_continente_inacabado::light::diorama;
use expedition33_continente_inacabado::scene::SpatialGroupId;
use expedition33_continente_inacabado::scenes::continent::blockout;

fn main() {
    let d = blockout();
    let s = d.scale.scene_radius;
    println!("  scene_radius = {s:.4}\n");

    for l in diorama(&d.anchors, &d.scale) {
        let grupos: Vec<&str> = SpatialGroupId::ALL
            .iter()
            .filter(|g| l.affects(**g))
            .map(|g| match g {
                SpatialGroupId::Global => "global",
                SpatialGroupId::ContinentBackground => "continente",
                SpatialGroupId::Meadows => "praderas",
                SpatialGroupId::Breakwater => "rompeolas",
                SpatialGroupId::FlyingWaters => "aguas",
                SpatialGroupId::Monolith => "monolito",
                SpatialGroupId::InteractionProps => "props",
            })
            .collect();

        println!(
            "  {}  intensidad {:.1}  range {:.3} ({:.2} S)",
            l.id,
            l.intensity,
            l.range,
            l.range / s
        );
        println!(
            "        posicion  ({:.2}, {:.2}, {:.2})",
            l.position.x, l.position.y, l.position.z
        );
        println!("        sombras   {}", l.casts_shadows);
        println!("        ilumina   {}", grupos.join(", "));
        println!(
            "        media/mitad a {:.3}  -> atenuacion {:.4}",
            l.range,
            l.attenuation(l.range)
        );
    }
}
