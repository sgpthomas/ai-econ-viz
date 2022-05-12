use three_d::{Color, Context, CpuMesh, Gm, Mesh, Model, PhysicalMaterial};

#[derive(Debug, Clone)]
pub enum Tile {
    Tree,
    Wall,
    Stone,
}

pub trait Meshable {
    fn mesh(&self, context: &Context) -> Gm<Mesh, PhysicalMaterial>;
    fn size(&self) -> (f32, f32) {
        (1.0, 1.0)
    }
}

impl Meshable for Tile {
    fn mesh(&self, context: &Context) -> Gm<Mesh, PhysicalMaterial> {
        match self {
            Tile::Tree => Model::new_with_material(
                &context,
                &CpuMesh::cone(20),
                PhysicalMaterial {
                    albedo: Color {
                        r: 0,
                        g: 200,
                        b: 0,
                        a: 100,
                    },
                    ..Default::default()
                },
            ),
            Tile::Wall => Model::new_with_material(
                &context,
                &CpuMesh::cube(),
                PhysicalMaterial {
                    albedo: Color {
                        r: 100,
                        g: 50,
                        b: 0,
                        a: 100,
                    },
                    ..Default::default()
                },
            ),
            Tile::Stone => Model::new_with_material(
                &context,
                &CpuMesh::sphere(20),
                PhysicalMaterial {
                    albedo: Color {
                        r: 50,
                        g: 50,
                        b: 50,
                        a: 100,
                    },
                    ..Default::default()
                },
            ),
        }
        .unwrap()
    }

    fn size(&self) -> (f32, f32) {
        match self {
            Tile::Tree => (0.2, 0.7),
            Tile::Wall => (0.1, 0.9),
            Tile::Stone => (0.05, 0.4),
        }
    }
}
