#[derive(Copy)]

pub struct Vertice {
    pub indice:usize
}

impl Clone for Vertice {
    fn clone(&self) -> Vertice { *self }
}