use std::fs::File;
use std::io::prelude::*;
use rand;

#[derive(Clone)]
struct Edge {
  vertices: [String; 2],
}

#[derive(Clone)]
struct Graph {
  edges: Vec<Edge>,
  vertices: Vec<String>,
}

fn read_file() -> Graph {
  let mut file = File::open("./input/input25.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut vertices = vec![];
  let mut edges = vec![];

  for line in contents.lines() {
    let sides: Vec<&str> = line.split(": ").collect();
    let (first, others) = sides.split_at(1);
    let others = others[0].split(" ");
    let first = first[0].to_owned();
    if !vertices.contains(&first) {
      vertices.push(first.clone());
    }
    for other in others {
      if !vertices.contains(&other.to_string()) {
        vertices.push(other.to_string());
      }

      let new_edge = Edge { vertices: [first.clone(), other.to_string()] };
      edges.push(new_edge);
    }
  }

  Graph { vertices, edges }
}

// going to use Karger's algorithm - https://en.wikipedia.org/wiki/Karger%27s_algorithm - to solve this.
// It isn't actually guaranteed to work (but has high probability), so will need to loop over various attempts until we
// succeed in reducing the graph to 2 vertices with 3 edges remaining between them.
// The trickiest thing will be getting the answer out. The easiest way will be to keep track of which "original" vertices
// the final ones "descend" from - and for that we'll name the new vertices by squashing together the names of the 2 which
// were merged. Since all vertices have a 3 letter name, we can then read of the size of the connected sets from the
// lengths of the names.

// The key will be the following function to contract an edge of the graph. We'll have it select a random one as that's
// all we'll use, and it simplifies the interface a lot!
fn contract_random_edge(graph: &mut Graph) {
  let random_number: usize = rand::random();
  let random_edge = graph.edges[random_number % graph.edges.len()].clone();

  let mut new_vertex = String::from(random_edge.vertices[0].clone());
  new_vertex.push_str(&random_edge.vertices[1]);

  // the new vertices consist of the old ones, but with the 2 end vertices of the selected edge removed, and a new
  // one added from their combination
  graph.vertices.retain(|vertex| !random_edge.vertices.contains(vertex));
  graph.vertices.push(new_vertex.clone());

  // the new edges are obtained from the old ones by removing any that joined the 2 ends of the removed edge (this
  // will always include the randomly-selected edge, but will probably include more as the algorithm proceeds), and
  // for any which had just one of the two ends as an end, replacing that end by the new vertex.
  graph.edges.retain(|edge| {
    !random_edge.vertices.contains(&edge.vertices[0])
    || !random_edge.vertices.contains(&edge.vertices[1])
  });

  for edge in &mut graph.edges.iter_mut() {
    for end_vertex in &mut edge.vertices.iter_mut() {
      if random_edge.vertices.contains(end_vertex) {
        *end_vertex = new_vertex.clone();
      }
    }
  }
}

// runs for a fair few seconds, because the algorithm takes a second or two per run and it fails a LOT
// of times. But generally it seems to get there in an acceptable time.
fn solve_part_1(graph: &mut Graph) -> u32 {
  loop {
    let mut copy = graph.clone();
    while copy.vertices.len() > 2 {
      contract_random_edge(&mut copy);
      if copy.edges.len() == 3 {
        let vertex_1 = &copy.vertices[0];
        let vertex_2 = &copy.vertices[1];
        return (vertex_1.chars().count() as u32 / 3) * (vertex_2.chars().count() as u32 / 3);
      }
    }
  } 
}

pub fn part_1() -> u32 {
  let mut graph = read_file();
  solve_part_1(&mut graph)
}
