use std::process::exit;

enum MenuChoices {
    GET_FIRSTNAME = 0,
    GET_LASTNAME = 1,
    GET_AGE = 2,
    CHECK_LEGAL = 3,
    CHANGE_FIRSTNAME = 4,
    CHANGE_LASTNAME = 5,
    CREATE_POST = 6,
    DISPLAY_ALL_POSTS = 7,
    EXIT = 9,
}

impl MenuChoices {
    fn from_value(menuEntry: i32) -> MenuChoices {
        match menuEntry {
            0 => MenuChoices::GET_FIRSTNAME,
            1 => MenuChoices::GET_LASTNAME,
            2 => MenuChoices::GET_AGE,
            3 => MenuChoices::CHECK_LEGAL,
            4 => MenuChoices::CHANGE_FIRSTNAME,
            5 => MenuChoices::CHANGE_LASTNAME,
            6 => MenuChoices::CREATE_POST,
            7 => MenuChoices::DISPLAY_ALL_POSTS,
            9 => MenuChoices::EXIT,
            _ => panic!("Index out of available options"),
        }
    }
}

struct PostsStore {
    posts: Vec<Post>,
}

impl PostsStore {
    fn create_post(&mut self, post_title: &String) -> () {
        let title = String::from(post_title);
        if title.trim().len() > 0 {
            let post = Post {
                author_id: 1,
                title,
                id: self.posts.len() as i32,
            };
            self.posts.push(post);
        }
    }

    fn new() -> PostsStore {
        PostsStore { posts: vec![] }
    }
}

struct Post {
    id: i32,
    title: String,
    author_id: i32,
}

struct User {
    firstname: String,
    lastname: String,
    age: i32,
}

impl User {
    fn change_firstname(&mut self, new_name: &String) -> () {
        let name = String::from(new_name);
        if name.trim().len() > 0 {
            self.firstname = name
        }
    }

    fn change_lastname(&mut self, new_name: &String) -> () {
        let name = String::from(new_name);
        if name.trim().len() > 0 {
            self.lastname = name
        }
    }

    fn is_legal(&self) -> bool {
        self.age >= 18
    }
}

fn handle_input_error(error: &std::io::Error) {
    println!("Error while providing input: {}", error);
    exit(1)
}

fn menu(user: &mut User, posts_store: &mut PostsStore) {
    let mut menuInput = String::new();

    println!("________________________________");
    println!("|  What do you want to do now? |");
    println!("|------------------------------|");
    println!("|   [0] Show your firstname    |");
    println!("|   [1] Show your lastname     |");
    println!("|   [2] Show your age          |");
    println!("|   [3] Check if u're legal    |");
    println!("|   [4] Change firstname       |");
    println!("|   [5] Change lastname        |");
    println!("|   [6] Create post            |");
    println!("|   [7] Display all posts      |");
    println!("|   [9] Exit                   |");
    println!("-------------------------------");

    match std::io::stdin().read_line(&mut menuInput) {
        Err(error) => println!("Error: {error}"),
        Ok(_) => {}
    }

    match menuInput.trim().parse::<i32>() {
        Err(error) => println!("Error while parsing string to number: {error}"),
        Ok(value) => match MenuChoices::from_value(value) {
            MenuChoices::GET_FIRSTNAME => println!("Your firstname: {}", user.firstname),
            MenuChoices::GET_LASTNAME => println!("Your lastname: {}", user.lastname),
            MenuChoices::GET_AGE => println!("Your age: {}", user.age),
            MenuChoices::CHECK_LEGAL => println!("Are you legal: {}", user.is_legal()),
            MenuChoices::CHANGE_FIRSTNAME => {
                let mut new_firstname = String::new();
                println!("Provide new firstname:");
                match std::io::stdin().read_line(&mut new_firstname) {
                    Ok(_) => {
                        user.change_firstname(&new_firstname);
                        println!("Successfully changed firstname!")
                    }
                    Err(error) => handle_input_error(&error),
                }
            }
            MenuChoices::CHANGE_LASTNAME => {
                let mut new_lastname = String::new();
                println!("Provide new lastname:");
                match std::io::stdin().read_line(&mut new_lastname) {
                    Ok(_) => {
                        user.change_lastname(&new_lastname);
                        println!("Successfully changed lastname!")
                    }
                    Err(error) => handle_input_error(&error),
                }
            }
            MenuChoices::CREATE_POST => {
                let mut post_name = String::new();
                println!("Provide post title:");
                match std::io::stdin().read_line(&mut post_name) {
                    Ok(_) => {
                        posts_store.create_post(&post_name);
                        println!(
                            "Successfully created post with id: {:?}",
                            posts_store.posts.last().unwrap().id
                        )
                    }
                    Err(error) => handle_input_error(&error),
                }
            }
            MenuChoices::DISPLAY_ALL_POSTS => {
                if posts_store.posts.is_empty() {
                    println!("No posts have been made yet")
                } else {
                    for post in &posts_store.posts {
                        println!("{}. {}", post.id, post.title)
                    }
                }
            }
            MenuChoices::EXIT => {
                print!("Goodbye, {}!", user.firstname);
                exit(0);
            }
        },
    }
}

fn main() {
    let mut firstname = String::new();
    let mut lastname = String::new();
    let mut ageInput = String::new();
    let mut age: i32 = 0;
    println!("Hi, before we start, provide me your firstname:");
    match std::io::stdin().read_line(&mut firstname) {
        Err(error) => handle_input_error(&error),
        _ => {}
    };
    println!("Now, provide your lastname:");
    match std::io::stdin().read_line(&mut lastname) {
        Err(error) => handle_input_error(&error),
        _ => {}
    }
    println!("Okay, now lastly provide your age:");
    match std::io::stdin().read_line(&mut ageInput) {
        Err(error) => handle_input_error(&error),
        _ => {}
    }

    match ageInput.trim().parse::<i32>() {
        Ok(n) => age = n,
        Err(error) => println!("Error while parsing {}error message: {}", ageInput, error),
    }

    let mut user = User {
        firstname,
        lastname,
        age,
    };

    println!("Successfully created user!");

    let mut posts_store = PostsStore::new();
    loop {
        menu(&mut user, &mut posts_store)
    }
}
