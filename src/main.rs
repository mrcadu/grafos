mod grafo;
use grafo::Grafo;

fn main() {
    let grafo = Grafo{arestas:[]};
    grafo.ler_arquivo_grafo();
}
