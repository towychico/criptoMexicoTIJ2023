use std::collections::HashMap;
pub struct Post{
    pub user_name: String,
    pub number_of_likes: u64,
    pub number_of_reports:u64,
    pub number_of_saves: u64,
    pub post_id:String,
    pub post_number:u64
}

pub struct Stack{
    pub stack:Vec<int>
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new()
        }
    }
    pub fn push(&mut self,element:u64){
            self.stack.push(element);

    }
    pub fn pop(&mut self) -> Option<T> {

            return self.stack.pop();


    }
    pub fn peek(&mut self) ->Option<&T>{
        let last_element:Option<&T> = self.stack.last();
        return last_element;

    }
    pub fn is_empty(&mut self) -> bool{
        if self.stack.is_empty(){
            return true;
        }else {
            return false
        }

    }
}
pub fn dfs(graph:Graph, start:u64) -> Vec<Option<T>> {
    let mut stack = Stack::new();
    stack.push(start);
    let mut result = Vec::new();
    let mut visited = HashMap::new();
    visited[start] = true;
    let mut current_vertex;

    stack.push(start);
    while !stack.is_empty() {
        current_vertex = stack.pop();
        result.push(current_vertex);

        for vertex in graph.adjacency_list[current_vertex]{
            if !visited[vertex] {
                visited[vertex] = true;
                stack.push(vertex)
            }
        }
    }
    return result;
}

pub struct Graph {
    adjacency_list: HashMap<u64, Vec<u64>>,
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, post: Post) {

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