use super::light::PointLight;
use super::material::*;
use super::renderable::*;
use super::triangle::Triangle;
use na::{Scalar, Vector3};
use obj::{IndexTuple, Obj, SimplePolygon};
use std::borrow::Borrow;
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
