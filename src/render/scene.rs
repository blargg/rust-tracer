use super::color::Rgb;
use super::light::PointLight;
use super::material::*;
use super::renderable::*;
use super::triangle::Triangle;
use super::ray::Ray;
use super::shape::Shape;
use na::{Scalar, Vector3};
use obj::{IndexTuple, Obj, SimplePolygon};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::path::Path;

type MatTri<T> = ShapeMat<Triangle<T>, UniformMaterial<Lambert<T>>>;

pub struct Scene<T: Scalar> {
    pub objects: Vec<MatTri<T>>,
    pub lights: Vec<PointLight<T>>,
}

#[derive(Debug)]
pub enum SceneLoadError {
    LoadObjError,
    SceneContainsGeneralPolyError,
}

impl<T: Scalar> Scene<T> {
    pub fn empty() -> Scene<T> {
        Scene {
            objects: vec![],
            lights: vec![],
        }
    }
}

// TODO generalize to GenFloat
impl Scene<f64> {

    pub fn intersects_renderable(&self, ray: &Ray<f64>) -> Option<(&MatTri<f64>, f64)> {
        let shape_inter = self.objects.iter().map(|s| (s, s.intersection(ray)));
        let closest = shape_inter.min_by(closest_to_ray(ray));
        closest.and_then(|(s, inter)| {
            match inter {
                None => None,
                Some(i) => Some((s, i)),
            }
        })
    }
}

// TODO check if this works
fn closest_to_ray<S: Renderable, T:PartialOrd>(ray: &Ray<S::NumTy>) -> fn (&(&S, Option<T>), &(&S, Option<T>)) -> Ordering {
    |(_, t1), (_, t2)| {
        match (t1, t2) {
            (None, _) => Ordering::Greater,
            (_, None) => Ordering::Less,
            (Some(x), Some(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        }
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
                    // use a standard color until we can load from the file
                    let material = UniformMaterial::new(Lambert::new(1.0, 0.0, 0.0));
                    scene.objects.push(MatTri::new(tri, material));
                }
            }
        }

        // hard coded lights
        let white = Rgb::new(1.0, 1.0, 1.0);
        let light: PointLight<f64> = PointLight { position: Vector3::new(5.0, 5.0, 1.0), color: white};
        scene.lights.push(light);

        Ok(scene)
    }
}

fn get_point(obj: &Obj<SimplePolygon>, point_index: IndexTuple) -> Vector3<f64> {
    let IndexTuple(pi, _, _) = point_index;
    let point = obj.position[pi];

    Vector3::new(
        f64::from(point[0]),
        f64::from(point[1]),
        f64::from(point[2]),
    )
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
