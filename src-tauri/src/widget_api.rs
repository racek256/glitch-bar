use warp::Filter;
use crate::widget_loader;
use serde::{Deserialize, Serialize};
use serde_json::Result;


#[tokio::main]
pub async fn widget_server(widget_path:&String){
    

    let widgets = widget_loader::load(&widget_path);


    let json = serde_json::to_string(&widgets);
    let json_str = json.unwrap();
    let routes = warp::path::end().map(move || json_str.clone());
    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}

