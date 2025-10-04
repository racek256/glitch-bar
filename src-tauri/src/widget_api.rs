use axum::{
    routing::{get,post,},
    http::StatusCode,
    extract::{State},
    Json, Router,
};
use serde::{Deserialize, Serialize};


use warp::filters::fs::file;
use crate::widget_loader::{self};
use serde_json::{Value, json};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path,fs, sync::{mpsc, Arc, Mutex}};
use tokio;




pub async fn widget_server(widget_path:String)->Result<()>{


    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&widget_path), RecursiveMode::Recursive)?;
    
    println!("Loading widgets: ");
    let widgets = Arc::new(Mutex::new(widget_loader::load(&widget_path)));

    let widgets_watch = Arc::clone(&widgets);
    let widgets_path_watch = Arc::new(widget_path.clone());

// Monitor for changes
    tokio::spawn(async move {
     for res in rx{
         match res {
             Ok(event) => {
                 match event.kind{ 
                     notify::event::EventKind::Modify(
                         notify::event::ModifyKind::Data(_)
                         ) => {
                         println!("Reloading Widgets: ");
                         let mut data = widgets_watch.lock().unwrap();
                         *data = widget_loader::load(&widgets_path_watch);


                     },
                     _=> {}
                 }
             },
             Err(e) => println!("watch error: {}", e)
         }
     }
    });



    


    let widgets_clone = widgets.clone();
    let widgets_path_state = Arc::new(widget_path.clone());


    // Create axum app
    let app = Router::new()
        .route("/", get(Json(json!(widgets_clone)))) 
        .route("/create",post(create_widget))
        .route("/modify",post(modify_widget))
        .with_state(AppState {
            filepath:widgets_path_state
        });

        



    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener,app).await.unwrap();

    Ok(())
}



async fn create_widget(
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


async fn modify_widget(
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
struct FileWriteStruct{
    html:String,
    filename:String,
}

#[derive(Clone)]
struct AppState{
    filepath:Arc<String>,
}

