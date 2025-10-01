use std::fs;
pub fn verify_config(path:&String){
    let os:String = std::env::consts::OS.to_string();
        let result = fs::exists(&path);
        match result {
            Ok(_) => {
                println!("config path: {}", path);
                println!("Config path already exists => No problem; ")
            },
            Err(_) => {
                println!("Config path doesnt  exists => Creating now => No Problem; ");
                let creation_result = fs::create_dir_all(&path);
                match creation_result {
                    Ok(_) => println!("Config folder succesfully created"),
                    Err(_) => {panic!("Failed to create Config folder")
                    },
                }
            }
        }
       




}
