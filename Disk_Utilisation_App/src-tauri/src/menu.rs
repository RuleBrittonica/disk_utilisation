use tauri::{CustomMenuItem, Menu, Submenu};

pub fn build_menu() -> Menu {
    // File submenu items
    let open = CustomMenuItem::new("open".to_string(), "Open...");
    let refresh = CustomMenuItem::new("refresh".to_string(), "Refresh");
    let refresh_all = CustomMenuItem::new("refresh_all".to_string(), "Refresh All");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    // Create the File submenu
    let file_submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(open)
            .add_item(refresh)
            .add_item(refresh_all)
            .add_item(quit),
    );

    // Edit Submenu options
    let copy_path = CustomMenuItem::new("copy_path".to_string(), "Copy Path");

    // Edit submenu
    let edit_submenu = Submenu::new(
        "Edit",
        Menu::new()
            .add_item(copy_path),
    );

    // Clean Up submenu options
    

    // Clean Up submenu
    let cleanup_submenu = Submenu::new("Clean Up", Menu::new());

    // Treemap submenu
    let treemap_submenu = Submenu::new("Treemap", Menu::new());

    // Report submenu
    let report_submenu = Submenu::new("Report", Menu::new());

    // Options submenu
    let options_submenu = Submenu::new("Options", Menu::new());

    // Help submenu
    let help_submenu = Submenu::new("Help", Menu::new());

    // Logs submenu
    let logs_submenu = Submenu::new("Logs", Menu::new());

    // Create the main menu and add the submenus
    Menu::new()
        .add_submenu(file_submenu)
        .add_submenu(edit_submenu)
        .add_submenu(cleanup_submenu)
        .add_submenu(treemap_submenu)
        .add_submenu(report_submenu)
        .add_submenu(options_submenu)
        .add_submenu(help_submenu)
        .add_submenu(logs_submenu)
}
