
#[derive(Debug)]
pub struct TrayModel {
    items: Vec<TrayItems>
}

#[derive(Debug)]
pub enum TrayItems {
    Tray(TrayModel),
    Tile(TileModel),
}

#[derive(Debug)]
pub struct TileModel {
    thing: String,
}

fn main() {
    let my_inner_tray_model = TrayModel{items: vec![
        TrayItems::Tile(TileModel{thing: String::from("hey")}),
        TrayItems::Tile(TileModel{thing: String::from("rust")}),
    ]};

    let my_tray_model = TrayModel{items: vec![
        TrayItems::Tile(TileModel{thing: String::from("hello")}),
        TrayItems::Tile(TileModel{thing: String::from("world")}),
        TrayItems::Tray(my_inner_tray_model),
    ]};

    for i in my_tray_model.items {
        println!("{:?}", i);
    }
}