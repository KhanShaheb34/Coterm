use dialoguer::Input;
use std::{
    env,
    fs::{self, create_dir_all, File},
    io::Write,
    path::Path,
};

fn get_env_path() -> String {
    let home = env::var("HOME").expect("Error getting HOME environment variable");
    let key_file = format!("{}/.coterm/key", home);
    key_file
}

pub fn get_api_key() -> String {
    match env::var("OPENAI_API_KEY") {
        Ok(_) => return env::var("OPENAI_API_KEY").unwrap(),
        Err(_) => {
            let key_file = get_env_path();
            if Path::new(&key_file).exists() {
                let api_key = fs::read_to_string(key_file.clone()).expect("Error reading key file");
                return api_key.clone();
            }

            println!("Please set the OPENAI_API_KEY environment variable. Get one at https://beta.openai.com/account/api-keys.");
            let api_key = Input::<String>::new()
                .with_prompt("OpenAI API Key")
                .interact_text()
                .expect("Error reading input");

            set_api_key(api_key.clone());
            return api_key.clone();
        }
    }
}

pub fn set_api_key(api_key: String) {
    let key_file = get_env_path();

    if let Some(parent_dir) = Path::new(&key_file).parent() {
        if !parent_dir.exists() {
            create_dir_all(parent_dir).expect("Error creating key file directory");
        }
    }

    let mut file = File::create(key_file.clone()).expect("Error creating key file");
    file.write_all(api_key.clone().as_bytes())
        .expect("Error writing to key file");

    println!("Key saved to {}", key_file.clone());
}
