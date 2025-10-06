use axum::{
    extract::{State, Path as UrlPath},
     Json,
    response::Html
};
use serde_json::{Value};
use serde::{Deserialize};
use std::{fs, sync::{ Arc, Mutex}};
use crate::widget_loader::{ WidgetStruct, PresetStruct};


pub async fn edit_preset(
    State(state):State<AppState>,
    Json(payload): Json<Value>
    )-> String{
    let location = format!("{}/{}.json", state.filepath, payload["name"]);
    println!("modifying preset at: {}", location);

    let result = fs::write(location, payload.to_string());

    match result{
        Ok(_) => return "Preset succesfully updated".to_string(),
        Err(e) => return format!("Error updating preset: {}", e)
    }; 
}


pub async fn get_presets(
    State(state):State<AppState>
    )->Json<Value>{
        
    let data = state.presets.lock().unwrap();
        let cloned_data = (*data).clone();
    Json(serde_json::to_value(cloned_data).unwrap())

}
pub async fn get_widget(
    State(state):State<AppState>,
    UrlPath(id):UrlPath<String>
    )->Html<String>{

    let filepath = format!("{}/{}.html",state.filepath,id);
    let exists = fs::exists(&filepath);
    match exists{
        Ok(true) => {},
        Ok(false) => return Html("Widget doesnt exist".to_string()),
        _ => ()
    };
    println!("{}",filepath);
    let html = fs::read_to_string(filepath);
    return Html(html.unwrap());
    
}

pub async fn get_widgets(
    State(state):State<AppState>
    )->Json<Value>{

    let data = state.widgets.lock().unwrap();
        let cloned_data = (*data).clone();

    Json(serde_json::to_value(cloned_data).unwrap())
    
}



pub async fn create_widget(
     State(state): State<AppState>,
     Json(payload): Json<FileWriteStruct>
    ) -> String{
    let new_file_path = format!("{}/{}.html", state.filepath, payload.filename);
    
    let exists = fs::exists(&new_file_path);
    match exists{
        Ok(false) => println!("new file will be created at: {} with code: {}", new_file_path, payload.html),
        Ok(true) => return "File already exists".to_string(),
        _ => println!("If you managed to trigger this error you are magician")

    };

    let response = fs::write(new_file_path, payload.html); 

    match response{
        Ok(_) => return "File created succesfully".to_string(),
        Err(e) => return format!("File failed to create with error: {}", e)
    }


}


pub async fn modify_widget(
     State(state): State<AppState>,
     Json(payload): Json<FileWriteStruct>
    ) -> String{
    let new_file_path = format!("{}/{}.html", state.filepath, payload.filename);
    
    let exists = fs::exists(&new_file_path);
    match exists{
        Ok(true) => println!("This file will be modified with code:  {} with code: {}", new_file_path, payload.html),
        Ok(false) => return "File doesnt exist yet please create it first".to_string(),
        _ => println!("If you managed to trigger this error you are magician")

    };

    let response = fs::write(new_file_path, payload.html); 

    match response{
        Ok(_) => return "File created succesfully".to_string(),
        Err(e) => return format!("File failed to create with error: {}", e)
    }


}





#[derive(Deserialize)]
pub struct FileWriteStruct{
    html:String,
    filename:String,
}

#[derive(Clone)]
pub struct AppState{
    pub filepath:Arc<String>,
    pub widgets:Arc<Mutex<Vec<WidgetStruct>>>,
    pub presets:Arc<Mutex<Vec<PresetStruct>>>
}


