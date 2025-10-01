use std::fs;
#[derive(serde::Serialize)]
pub struct WidgetStruct {
    widget_name:String,
    html:String
}

pub fn load(filepath:&String) -> Vec<WidgetStruct> { // rename later
    let paths = fs::read_dir(filepath).unwrap();
    let mut widgets:Vec<WidgetStruct> = Vec::new();
    for path in paths{
        let path_buf = path.unwrap().path();
        let file_path = path_buf.display();
        let file_path_string = path_buf.to_string_lossy();
        println!("name: {} ", file_path);
        if file_path_string.contains(".html") {
         let widget = fs::read_to_string(path_buf);
        widgets.push(WidgetStruct { widget_name: "Blank".to_string(), html: widget.unwrap() });
   
        }
            }
    for widget in &widgets{
        println!("widget: {}", widget.html);

    }
    return widgets;
    }
