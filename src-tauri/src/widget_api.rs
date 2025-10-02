use warp::Filter;
use crate::widget_loader;
use serde_json;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::{mpsc, Arc, Mutex}};
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
let routes = warp::path::end().map(move || {
    let data = widgets_clone.lock().unwrap();
    warp::reply::json(&*data) // returns proper JSON reply
});
    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;

    Ok(())
}

