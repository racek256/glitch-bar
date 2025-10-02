use std::fs;
#[derive(serde::Serialize)]

pub struct WidgetStruct {
    widget_name:String,
    html:String,
}

pub fn load(filepath:&String) -> Vec<WidgetStruct> { 
    let paths = fs::read_dir(filepath).unwrap();
    let mut widgets:Vec<WidgetStruct> = Vec::new();
    for path in paths{
        let path_buf = path.unwrap().path();
        let file_path_string = path_buf.to_string_lossy();

        if file_path_string.contains(".html") {
         let widget = fs::read_to_string(&path_buf);
        widgets.push(WidgetStruct { html: widget.unwrap(), widget_name: path_buf.file_stem().unwrap().to_string_lossy().to_string()});

        }
            }
    for widget in &widgets{
        println!(" - loaded widget: {}", widget.widget_name);

    }
    return widgets;
    }
