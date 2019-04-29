extern crate cgmath;
extern crate image;
extern crate obj;

use super::ray::Ray;
use super::renderable::Renderable;
use super::triangle::Triangle;
use cgmath::{vec3, Vector3};
use image::{ImageBuffer, Pixel, Rgb};
use obj::{IndexTuple, Obj, SimplePolygon};
use std::borrow::Borrow;
use std::path::Path;

pub struct Scene<T> {
    objects: Vec<Triangle<T>>,
}

#[derive(Debug)]
pub enum SceneLoadError {
    LoadObjError,
    SceneContainsGeneralPolyError,
}

fn get_point(obj: &Obj<SimplePolygon>, point_index: IndexTuple) -> Vector3<f64> {
    let IndexTuple(pi, _, _) = point_index;
    let point = obj.position[pi];

    vec3(point[0] as f64, point[1] as f64, point[2] as f64)
}

fn to_triangle(
    obj: &Obj<SimplePolygon>,
    poly: &SimplePolygon,
) -> Result<Triangle<f64>, SceneLoadError> {
    if poly.len() != 3 {
        return Err(SceneLoadError::SceneContainsGeneralPolyError);
    }
    Ok(Triangle::new(
        get_point(obj, poly[0]),
        get_point(obj, poly[1]),
        get_point(obj, poly[2]),
    ))
}

impl<T> Scene<T> {
    pub fn empty() -> Scene<T> {
        Scene { objects: vec![] }
    }
}

impl Scene<f64> {
    pub fn load(path: &Path) -> Result<Scene<f64>, SceneLoadError> {
        let obj: Obj<SimplePolygon> = Obj::load(path).map_err(|_| SceneLoadError::LoadObjError)?;
        let mut scene: Scene<f64> = Scene::empty();

        let objects: &Vec<_> = obj.objects.borrow();
        for object in objects {
            let groups: &Vec<_> = object.groups.borrow();
            for group in groups {
                let polys: &Vec<_> = group.polys.borrow();
                for poly in polys {
                    let tri = to_triangle(&obj, poly)?;
                    scene.objects.push(tri);
                }
            }
        }
        Ok(scene)
    }
}

pub fn render(scene: &Scene<f64>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img_height = 100;
    let img_width = 100;
    let scene_height = 100.0;
    let scene_width = 100.0;

    ImageBuffer::from_fn(img_height, img_width, |x, y| {
        render_ray(generate_ray(x, y), &scene)
    })
}

fn generate_ray(x: u32, y: u32) -> Ray<f64> {
    let origin = vec3(x as f64, y as f64, 0.0);
    let direction = vec3(0.0, 0.0, 1.0);
    Ray::new(origin, direction)
}

fn render_ray(ray: Ray<f64>, scene: &Scene<f64>) -> Rgb<u8> {
    let objs: &Vec<_> = scene.objects.borrow();
    for sphere in objs {
        if sphere.intersection(&ray).is_some() {
            return Rgb::from_channels(255u8, 0, 0, 255);
        }
    }
    return Rgb::from_channels(0u8, 0, 0, 255);
}
