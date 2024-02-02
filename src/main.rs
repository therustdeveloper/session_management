mod method;

use std::collections::HashMap;
use rand::Rng;

fn main() {
    println!("Rust Session Management!");
}

// Create the underlying struct for the User.
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

// Implementing the User struct.
impl User {
    // new is a constructor function for User that creates a new user.
    pub fn new(id: u32, username: &str, email: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string()
        }
    }

    // function to get a user by ID from the simulated user database.
    pub fn get_user_by_id(user_id: u32, user_db: &HashMap<u32, User>) -> Option<&User> {
        user_db.get(&user_id)
    }

    // function to update a user's credential in the user database.
    pub fn update_user_credential(user_id: u32, new_username: &str, new_email: &str, user_db: &mut HashMap<u32, User>) -> bool {
        if let Some(user) = user_db.get_mut(&user_id) {
            user.username = new_username.to_string();
            user.email = new_email.to_string();
            true
        } else {
            false
        }
    }

    // method to display the details of current user instance.
    pub fn display_user_details(&self) {
        println!("ID: {}, Username: {}, Email: {}", self.id, self.username, self.email);
    }
}

// Create the underlying struct for the Session.
#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: u32,
    pub token: String,
}

// Implement the Session struct.
impl Session {
    // new is a constructor function for Session that creates a new user session.
    pub fn new(user_id: u32) -> Self {
        let token = rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        Session { user_id, token }
    }
}
