
pub mod areas_volumes;
use crate::areas_volumes::{GeometricalShapes, GeometricalVolumes};
use crate::areas_volumes::{
    square_area, triangle_area, circle_area, rectangle_area,
    cube_volume, sphere_volume, triangular_pyramid_volume,
    parallelepiped_volume, cone_volume
};

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,  
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let fits = match objects {
        GeometricalShapes::Square => a <= x && a <= y,
        GeometricalShapes::Circle => 2 * a <= x && 2 * a <= y,
        GeometricalShapes::Rectangle => (a <= x && b <= y) || (b <= x && a <= y),
        GeometricalShapes::Triangle => (a <= x && b <= y) || (b <= x && a <= y),
    };
    if !fits {
        return false;
    }
    let container_area = x as f64 * y as f64;
    let object_area = match objects {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };
    object_area * times as f64 <= container_area
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,  // No need for module prefix
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let box_volume = x as f64 * y as f64 * z as f64;
    let total_volume = match objects {
        GeometricalVolumes::Cube => cube_volume(a) as f64 * times as f64,
        GeometricalVolumes::Sphere => sphere_volume(a) * times as f64,
        GeometricalVolumes::Cone => cone_volume(a, b) * times as f64,
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64, b) * times as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64 * times as f64,
    };
    total_volume <= box_volume
}