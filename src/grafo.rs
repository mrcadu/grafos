use self::super::aresta::Aresta;
use std::fs::File;
use std::io::prelude::*;

pub struct Grafo {
    arestas:Vec<Aresta>
}

impl Grafo {
    pub fn ler_arquivo_grafo(&self) {
        let mut arquivo_grafo = File::open("arquivoGrafo.txt").expect("Unable to open");
        let mut contents = String::new();
        arquivo_grafo.read_to_string(&mut contents);
    }
}