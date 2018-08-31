#[derive(Copy)]

pub struct Aresta {
    pub vertice_origem: usize,
    pub vertice_destino:usize
}

impl Clone for Aresta {
    fn clone(&self) -> Aresta { *self }
}