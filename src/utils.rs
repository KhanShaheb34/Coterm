use dialoguer::Input;
use std::{
    env,
    fs::{self, create_dir_all, File},
    io::Write,
    path::Path,
};

pub fn manage_environment_variables() {
    match env::var("OPENAI_API_KEY") {
        Ok(_) => {}
        Err(_) => {
            let home = env::var("HOME").expect("Error getting HOME environment variable");
            let key_file = format!("{}/.coterm/key", home);
            if Path::new(&key_file).exists() {
                let api_key = fs::read_to_string(key_file.clone()).expect("Error reading key file");
                env::set_var("OPENAI_API_KEY", api_key);
                return;
            }

            println!("Please set the OPENAI_API_KEY environment variable. Get one at https://beta.openai.com/account/api-keys.");
            let api_key = Input::<String>::new()
                .with_prompt("OpenAI API Key")
                .interact_text()
                .expect("Error reading input");
            env::set_var("OPENAI_API_KEY", api_key.clone());

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
    }
}
