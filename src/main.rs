mod models;
use crate::models::PostsStore;
use crate::models::User;

use std::process::exit;
use uuid::Uuid;
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

fn handle_input_error(error: &std::io::Error) {
    eprintln!("Error while providing input: {}", error);
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
    println!("|   [3] Check if you're legal  |");
    println!("|   [4] Change firstname       |");
    println!("|   [5] Change lastname        |");
    println!("|   [6] Create post            |");
    println!("|   [7] Display all posts      |");
    println!("|   [9] Exit                   |");
    println!("-------------------------------");

    if let Err(error) = std::io::stdin().read_line(&mut menuInput) {
        eprintln!("Error: {error}")
    }

    match menuInput.trim().parse::<i32>() {
        Err(error) => eprintln!("Error while parsing string to number: {error}"),
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
                        posts_store.create_post(&user, &post_name);
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
                    for (index, post) in posts_store.posts.iter().enumerate() {
                        println!("{}. {}", index, post.title)
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
    if let Err(error) = std::io::stdin().read_line(&mut firstname) {
        handle_input_error(&error)
    }

    println!("Now, provide your lastname:");
    if let Err(error) = std::io::stdin().read_line(&mut lastname) {
        handle_input_error(&error)
    }

    println!("Okay, now lastly provide your age:");
    if let Err(error) = std::io::stdin().read_line(&mut ageInput) {
        handle_input_error(&error)
    }

    match ageInput.trim().parse::<i32>() {
        Ok(n) => age = n,
        Err(error) => eprintln!("Error while parsing {}error message: {}", ageInput, error),
    }

    let mut user = User {
        firstname,
        lastname,
        age,
        id: Uuid::new_v4(),
    };

    println!("Successfully created user!");

    let mut posts_store = PostsStore::new();
    loop {
        menu(&mut user, &mut posts_store)
    }
}
