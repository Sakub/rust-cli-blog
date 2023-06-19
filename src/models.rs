use uuid::Uuid;

pub struct PostsStore {
    pub posts: Vec<Post>,
}

impl PostsStore {
    pub fn create_post(&mut self, user: &User, post_title: &str) {
        if !post_title.trim().is_empty() {
            let post = Post {
                author_id: user.id,
                title: post_title.to_string(),
                id: Uuid::new_v4(),
            };
            self.posts.push(post);
        }
    }

    pub fn new() -> PostsStore {
        PostsStore { posts: vec![] }
    }
}

pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub author_id: Uuid,
}

pub struct User {
    pub firstname: String,
    pub lastname: String,
    pub age: i32,
    pub id: Uuid,
}

impl User {
    pub fn change_firstname(&mut self, new_name: &str) {
        let name = String::from(new_name);
        if !name.trim().is_empty() {
            self.firstname = name
        }
    }

    pub fn change_lastname(&mut self, new_name: &str) {
        let name = String::from(new_name);
        if !name.trim().is_empty() {
            self.lastname = name
        }
    }

    pub fn is_legal(&self) -> bool {
        self.age >= 18
    }
}
