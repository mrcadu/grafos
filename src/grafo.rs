use self::super::aresta::Aresta;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;
use std::collections::LinkedList;

pub struct Grafo {
    pub arestas:Vec<LinkedList<Aresta>>
}

pub struct GrafoBuilder{
}

impl GrafoBuilder{
    pub fn ler_arquivo_grafo(){
        let mut arquivo_grafo;
        let mut arquivo_grafo_buffer;
        let mut grafo_como_string = String::new();
        let mut linhas_arquivo_grafo : Vec<&str>;
        let numero_vertices;
        let mut vetor_arestas : Vec<LinkedList<Aresta>>;

        arquivo_grafo = File::open("arquivoGrafo.txt").expect("arquivo grafo incorreto");
        arquivo_grafo_buffer = BufReader::new(arquivo_grafo);
        arquivo_grafo_buffer.read_to_string(&mut grafo_como_string).expect("impossivel tornar arquivo uma string");
        linhas_arquivo_grafo = grafo_como_string.split("\n").collect();
        numero_vertices = linhas_arquivo_grafo[0].parse::<usize>().unwrap();
        linhas_arquivo_grafo.remove(0);
        vetor_arestas = Vec::with_capacity(numero_vertices);
        for linha_arquivo_grafo in linhas_arquivo_grafo {
            let vertices_aresta:Vec<&str> = linha_arquivo_grafo.split(" ").collect();
            let vertice_origem = vertices_aresta[0].parse::<u64>().unwrap();
            let vertice_destino = vertices_aresta[1].parse::<u64>().unwrap();
            let aresta = Aresta{ vertice_origem, vertice_destino};
            //vetor_arestas[vertice_origem].push_back(aresta);
            //vetor_arestas[vertice_origem].push_back(vertice_destino);
            //vetor_arestas[vertice_destino].push_back(vertice_origem);
        }
    }
}