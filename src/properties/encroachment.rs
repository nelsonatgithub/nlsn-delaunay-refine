use crate::elements::vertex::*;
use crate::properties::continence::*;

pub fn encroach(v1: &Vertex, v2: &Vertex, vertex: &Vertex) -> Continence {
    let x = vertex.x;
    let y = vertex.y;
    let x1 = v1.x;
    let y1 = v1.y;
    let x2 = v2.x;
    let y2 = v2.y;

    let measure = (x - x2) * (x - x1) + (y - y2) * (y - y1);

    if float_cmp::approx_eq!(f64, measure, 0.0, epsilon = 1.0E-14f64) {
        return Continence::Boundary;
    }

    if measure > 0.0 {
        return Continence::Outside;
    } else {
        return Continence::Inside;
    }
}

#[cfg(test)]
mod encroach {
    use super::*;

    #[test]
    fn test_inside() {
        let v1 = Vertex::new(0.0, 0.0);
        let v2 = Vertex::new(1.0, 1.0);

        let trial_vertex = Vertex::new(0.0, 0.99);

        assert_eq!(encroach(&v1, &v2, &trial_vertex), Continence::Inside);
    }

    #[test]
    fn test_outside() {
        let v1 = Vertex::new(0.0, 0.0);
        let v2 = Vertex::new(1.0, 1.0);

        let trial_vertex = Vertex::new(0.0, 1.01);

        assert_eq!(encroach(&v1, &v2, &trial_vertex), Continence::Outside);
    }

    #[test]
    fn test_boundary() {
        let v1 = Vertex::new(0.0, 0.0);
        let v2 = Vertex::new(1.0, 1.0);

        let trial_vertex = Vertex::new(0.0, 1.0);

        assert_eq!(encroach(&v1, &v2, &trial_vertex), Continence::Boundary);
    }
}
