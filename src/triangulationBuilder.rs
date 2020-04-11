use crate::continence::*;
use crate::orientation::*;
use crate::triangle::*;
use crate::vertex::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use std::rc::Rc;
use std::fmt;

/* Triangulator will build the triangulation by inserting triangles 
and removing vertices.

    - It starts by creating vertices from vector coordinates and
    choosing three vertices to compose the first triangle.
    - For each new triangle, a conflict is searched.
    - While there is a conflict, it resolves the conflict.
    - While there is a vertex left inserting, it inserts it.

At the end, there should be no vertex left inserting and no conflict 
left resolving. The triangles will detain vertices and coordinates.

A triangle and a vertex are in conflict if the vertex is located
inside the circuncircle of the triangle.  */

pub struct Triangulator {
    vertices: Vec<Rc<Vertex>>,
    triangles: HashSet<Triangle>,
    conflict_map: Vec<(Triangle, Rc<Vertex>)>,
    adjacency: HashMap<(Rc<Vertex>, Rc<Vertex>), Rc<Vertex>>
}

impl fmt::Display for Triangulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vertices\n");
        for vertex in self.vertices.iter() {
            write!(f, "{}\n", vertex);
        }
        write!(f, "\nTriangles\n");
        for triangle in self.triangles.iter() {
            write!(f, "{}\n", triangle);
        }
        write!(f, "\nConflicts\n");
        for (triangle, vertex) in self.conflict_map.iter() {
            write!(f, "{} -> {}\n", triangle, vertex);
        }
        
        return write!(f, "");
    }
}

impl Triangulator {
    /*
       TODO:
           - implement constrained delaunay triangulation
           - include segments as constraint
    */
    pub fn from_vertices(vertices_coordinates: Vec<f64>) -> Triangulator {
        Triangulator {
            vertices: Vertex::from_coordinates(vertices_coordinates),
            triangles: HashSet::new(),
            conflict_map: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    fn init(&mut self) {
        let ghost_vertex = Rc::new(Vertex::new_ghost(self.vertices.len()));

        let mut v3 = self.vertices.pop().unwrap();
        let mut v2 = self.vertices.pop().unwrap();
        let mut v1 = self.vertices.pop().unwrap();

        /* Loops until 3 non colinear vertices are found */
        loop {
            match orient_2d(&v1, &v2, &v3) {
                Orientation::Counterclockwise => {
                    break;
                }
                Orientation::Clockwise => {
//                    mem::swap(&mut v2, &mut v3);
                    break;
                }
                Orientation::Colinear => {
                    self.vertices.insert(0, v3);
                    v3 = self.vertices.pop().unwrap();
                }
            }; /* end - match orient_2d */
        } /* end - loop */

        let solid_triangle = Triangle::new(&v1, &v2, &v3);
        let tghost_1 = Triangle::new(&v1, &v2, &ghost_vertex);
        let tghost_2 = Triangle::new(&v2, &v3, &ghost_vertex);
        let tghost_3 = Triangle::new(&v3, &v1, &ghost_vertex);

        self.handle_triangle(solid_triangle);
        self.handle_triangle(tghost_1);
        self.handle_triangle(tghost_2);
        self.handle_triangle(tghost_3);
    }

    fn handle_conflict(&mut self) {
//        let (triangle, vertex) = self.conflict_map.pop().unwrap();

    }

    fn handle_triangle(&mut self, triangle: Triangle) {
        match self.vertices.iter().position(|vertex| {
            /* searchs for conflicting vertex */
            triangle.encircles(vertex) == Continence::Inside
        }) {
            Some(index) => {
                let conflicting_vertex = self.vertices.remove(index);
                self.conflict_map.push((triangle, conflicting_vertex));
            }
            None => {
                self.triangles.insert(triangle);
            }
        }
    }
}

#[cfg(test)]
mod constructor {
    use super::*;

    #[test]
    fn test_constructor() {
        let mut vertex_indices = Vec::new();
        vertex_indices.push(0.0);
        vertex_indices.push(0.0);
        vertex_indices.push(2.0);
        vertex_indices.push(0.0);
        vertex_indices.push(1.0);
        vertex_indices.push(2.0);
        let builder = Triangulator::from_vertices(vertex_indices);
        assert_eq!(builder.vertices.len(), 3);
    }

    #[test]
    fn test_init_single_triangle() {
        let mut vertex_indices = Vec::new();
        vertex_indices.push(0.0); vertex_indices.push(0.0);
        vertex_indices.push(2.0); vertex_indices.push(0.0);
        vertex_indices.push(1.0); vertex_indices.push(2.0);
        let mut builder = Triangulator::from_vertices(vertex_indices);
        builder.init();
        println!("{}", builder);
        assert_eq!(builder.vertices.len(), 0);
        assert_eq!(builder.triangles.len(), 4);
    }

    #[test]
    fn test_init_triangle_with_conflict() {
        let mut vertex_indices = Vec::new();
        vertex_indices.push(0.0); vertex_indices.push(0.0);
        vertex_indices.push(2.0); vertex_indices.push(0.0);
        vertex_indices.push(1.0); vertex_indices.push(2.0);
        vertex_indices.push(1.0); vertex_indices.push(1.0);
        let mut builder = Triangulator::from_vertices(vertex_indices);
        builder.init();
        println!("{}", builder);
        assert_eq!(builder.vertices.len(), 0);
        assert_eq!(builder.triangles.len(), 3);
        assert_eq!(builder.conflict_map.len(), 1);
    }
}
