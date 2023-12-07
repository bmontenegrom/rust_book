use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for launch today");

    assert_eq!("", post.content());
}
