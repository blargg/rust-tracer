use super::triangle::Triangle;
use cgmath::{vec3, Vector3};
use obj::{IndexTuple, Obj, SimplePolygon};
use std::borrow::Borrow;
use std::path::Path;

pub struct Scene<T> {
    pub objects: Vec<Triangle<T>>,
}

#[derive(Debug)]
pub enum SceneLoadError {
    LoadObjError,
    SceneContainsGeneralPolyError,
}

fn get_point(obj: &Obj<SimplePolygon>, point_index: IndexTuple) -> Vector3<f64> {
    let IndexTuple(pi, _, _) = point_index;
    let point = obj.position[pi];

    vec3(f64::from(point[0]), f64::from(point[1]), f64::from(point[2]))
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
