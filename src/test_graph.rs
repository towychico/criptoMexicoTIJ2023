
#[macro_use(c)]
use crate::models::{Graph, Post, Stack};

fn main() {
    let mut post1 = Post{user_name:"Lou".to_string(),number_of_likes:0,number_of_reports:0,number_of_saves:0,post_id:"A01".to_string(),post_number:2};
    let mut post2 = Post{user_name:"Mir".to_string(),number_of_likes:0,number_of_reports:0,number_of_saves:0,post_id:"A02".to_string(),post_number:2};
    /*let mut wa = c![x, for x in 0..100, if x%2==0];*/
   /* println!("{:?}",wa)*/
    let mut mir = Graph::new();
    mir.add_node(&post1);
    mir.add_node(&post2);

    let mut test = Stack::new();
    test.push(&post1);
    test.pop();
    mir.add_vertex(&post1,&post2);
    if let Some(adj_posts) = mir.get_adjacent_posts(&post1) {
        println!("{:?}", adj_posts)
    }


}
