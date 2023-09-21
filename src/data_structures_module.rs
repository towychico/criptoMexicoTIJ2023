use std::collections::HashMap;
pub struct Post{
    pub user_name: String,
    pub number_of_likes: u64,
    pub number_of_reports:u64,
    pub number_of_saves: u64,
    pub post_id:String,
    pub post_number:u64


}

/*pub struct Graph{
    adjacency_list: HashMap<u64, Vec<Post>>
}*/

// impl Graph {
//     pub fn new() -> Self {
//         Graph {
//             adjacency_list: HashMap::new(),
//         }
//     }
//
//     pub fn add_node(&mut self, post: Post) {
//         // Add a node (post) to the graph
//         self.adjacency_list.insert(post.post_number, Vec::new());
//     }
//     pub fn add_vertex(&mut self, from_post_number: u64, to_post_number: u64,post2:&Post) {
//         // Add an edge (vertex) between two posts
//         if let Some(adj_list) = self.adjacency_list.get_mut(&from_post_number) {
//             adj_list.push(post2);
//         }
//     }
//
//     pub fn get_adjacent_posts(&self, post_number: u64) -> Option<&Vec<Post>> {
//         // Get the list of adjacent posts for a given post number
//         self.adjacency_list.get(&post_number)
//     }
//
// }


/*impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, post: Post) {
        // Add a node (post) to the graph
        self.adjacency_list.insert(post.post_number, Vec::new());
    }

    pub fn add_vertex(&mut self, from_post_number: u64, to_post_number: u64) {
        // Add an edge (vertex) between two posts
        if let Some(adj_list) = self.adjacency_list.get_mut(&from_post_number) {
            adj_list.push(to_post_number);
        }
    }

    pub fn get_adjacent_posts(&self, post_number: u64) -> Option<&Vec<u64>> {
        // Get the list of adjacent posts for a given post number
        self.adjacency_list.get(&post_number)
    }
}
*//*impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, post: Post) {
        // Add a node (post) to the graph
        self.adjacency_list.insert(post.post_number, Vec::new());
    }

    pub fn add_vertex(&mut self, from_post_number: u64, to_post_number: u64) {
        // Add an edge (vertex) between two posts
        if let Some(adj_list) = self.adjacency_list.get_mut(&from_post_number) {
            adj_list.push(to_post_number);
        }
    }

    pub fn get_adjacent_posts(&self, post_number: u64) -> Option<&Vec<u64>> {
        // Get the list of adjacent posts for a given post number
        self.adjacency_list.get(&post_number)
    }
}*/


pub struct Graph {
    adjacency_list: HashMap<u64, Vec<u64>>, // Use Vec<u64> to represent adjacent post numbers
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, post: Post) {
        // Add a node (post) to the graph
        self.adjacency_list.insert(post.post_number, Vec::new());
    }

    pub fn add_vertex(&mut self, from_post_number: u64, to_post_number: u64) {
        // Add an edge (vertex) between two posts
        if let Some(adj_list) = self.adjacency_list.get_mut(&from_post_number) {
            adj_list.push(to_post_number);
        }
    }

    pub fn get_adjacent_posts(&self, post_number: u64) -> Option<&Vec<u64>> {
        // Get the list of adjacent posts for a given post number
        self.adjacency_list.get(&post_number)
    }
}