use std::fs;
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct WidgetStruct {
    widget_name:String,
    html:String,
}
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct PresetStruct {
    filename:String,
    name:String, 
}


pub fn load_widgets(filepath:&String) -> Vec<WidgetStruct> { 
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


pub fn load_presets(filepath:&String) -> Vec<PresetStruct> { 
    let paths = fs::read_dir(filepath).unwrap();
    let mut presets:Vec<PresetStruct> = Vec::new();
    for path in paths{
        let path_buf = path.unwrap().path();
        let file_path_string = path_buf.to_string_lossy();

        if file_path_string.contains(".json") {
        presets.push(PresetStruct {name: path_buf.file_stem().unwrap().to_string_lossy().to_string(), filename:path_buf.file_stem().unwrap().to_string_lossy().to_string() });

        }
            }
    for preset in &presets{
        println!(" - loaded preset: {}", preset.name);

    }
    return presets;
    }
