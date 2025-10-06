use axum::{
    http::{HeaderValue, Method}, 
    routing::{get,post,}, Router,
};
use crate::router::{get_widgets, create_widget, edit_preset, modify_widget, get_widget, get_presets, AppState, FileWriteStruct};
use serde::{Deserialize};
use tower_http::cors::{CorsLayer};


use crate::widget_loader::{self, WidgetStruct, PresetStruct};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::{mpsc, Arc, Mutex}};
use tokio;




pub async fn widget_server(widget_path:String)->Result<()>{


    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&widget_path), RecursiveMode::Recursive)?;
    
    println!("Loading widgets: ");
    let widgets = Arc::new(Mutex::new(widget_loader::load_widgets(&widget_path)));
    let presets = Arc::new(Mutex::new(widget_loader::load_presets(&widget_path)));
    
    let widgets_watch = Arc::clone(&widgets);
    let presets_watch = Arc::clone(&presets);
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
                         *data = widget_loader::load_widgets(&widgets_path_watch);
                         let mut preset_data = presets_watch.lock().unwrap();
                         *preset_data = widget_loader::load_presets(&widgets_path_watch)

                     },
                     _=> {}
                 }
             },
             Err(e) => println!("watch error: {}", e)
         }
     }
    });



    


    let widgets_path_state = Arc::new(widget_path.clone());
        let app = Router::new()
        .route("/", get(get_widgets)) 
        .route("/widgets", get(get_widgets)) 
        .route("/create/widget",post(create_widget))
        .route("/modify/preset",post(edit_preset))
        .route("/presets",post(get_presets))
        .route("/modify/widget",post(modify_widget))
        .route("/widgets/{id}",get(get_widget))
        .with_state(AppState {
            filepath:widgets_path_state,
            widgets:widgets,
            presets:presets
        }).layer(
        CorsLayer::new()
            .allow_origin("http://localhost:1420".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET]),
        );



        



    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener,app).await.unwrap();

    Ok(())
}

