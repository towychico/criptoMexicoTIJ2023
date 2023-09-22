pub mod data_structures_module;
mod test_graph;
mod models;

use crate::data_structures_module::Post;
fn main() {

    let post1 = Post {
        user_name: "Lou".to_string(),
        number_of_likes: 2,
        number_of_reports: 0,
        number_of_saves: 0,
        post_id: "A01723713".to_string(),
        post_number: 0,

    };
    let post2 = Post {
        user_name: "Mir".to_string(),
        number_of_likes: 3,
        number_of_reports: 0,
        number_of_saves: 0,
        post_id: "A01723713".to_string(),
        post_number: 1,

    };
    let post3 = Post {
        user_name: "Alvarin".to_string(),
        number_of_likes: 2,
        number_of_reports: 0,
        number_of_saves: 0,
        post_id: "A01723713".to_string(),
        post_number: 2,

    };

    let post4 = Post {
        user_name: "Jhonny".to_string(),
        number_of_likes: 2,
        number_of_reports: 0,
        number_of_saves: 0,
        post_id: "A01723713".to_string(),
        post_number: 3,

    };

    let post5 = Post {
        user_name: "Kevo".to_string(),
        number_of_likes: 10,
        number_of_reports: 0,
        number_of_saves: 0,
        post_id: "A01723713".to_string(),
        post_number: 4,

    };
    let mut graph = Graph::new();

    graph.add_node(post1);
    graph.add_node(post2);
    graph.add_node(post3);
    graph.add_node(post4);
    graph.add_node(post5);
    graph.add_vertex(0,1);
    graph.add_vertex(1,2);
    graph.add_vertex(1,3);
    graph.add_vertex(3,4);


    if let Some(adj_posts) = graph.get_adjacent_posts(1) {
        println!("{:?}", adj_posts)
    }




}
use data_structures_module::Graph;
