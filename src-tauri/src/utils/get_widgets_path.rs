use dirs_next as dirs;
pub fn get_widgets_path()-> String{
    let os:String = std::env::consts::OS.to_string();
    if os == "linux"{
        return dirs::home_dir().unwrap().join(".local/share/glitch-bar").to_string_lossy().to_string()       
    }else if os =="android"{
        // sorry android but you gotta wait
    }else if os == "windows"{
        // Fuck you windows
    }else{
        
    }


    panic!("failed to find default config path for your os");
}
