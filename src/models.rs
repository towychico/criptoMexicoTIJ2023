    use std::collections::HashMap;
    pub struct Post {
        pub user_name: String,
        pub number_of_likes: u64,
        pub number_of_reports: u64,
        pub number_of_saves: u64,
        pub post_id: String,
        pub post_number: u64
    }

    pub struct Graph {
        adjacency_list: HashMap<String, Vec<String>>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                adjacency_list: HashMap::new(),
            }
        }
        pub fn add_node(&mut self, post: &Post) {

            self.adjacency_list.insert((*post.post_id).parse().unwrap(), Vec::new());
        }
        pub fn add_vertex(&mut self, from_post: &Post, to_post: &Post) {
            // Add an edge (vertex) between two posts
            if let Some(adj_list) = self.adjacency_list.get_mut(&from_post.post_id) {
                adj_list.push(to_post.post_id.clone());
            }
        }
        pub fn get_adjacent_posts(&self, post: &Post) -> Option<&Vec<String>> {
            // Get the list of adjacent posts for a given post number
            return self.adjacency_list.get(&post.post_id)
        }
    }

    pub struct Stack {
        pub stack:Vec<Post>
    }

