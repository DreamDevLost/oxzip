// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    error::Error,
    rc::Rc,
    sync::atomic::{AtomicI32, AtomicU64},
};

use chrono::{DateTime, Local};
use humansize::{format_size, WINDOWS};
use slint::{Model, ModelRc, SharedString, StandardListViewItem, VecModel};

slint::include_modules!();

macro_rules! str2StandardListViewItem {
    ($s:expr) => {
        StandardListViewItem::from(SharedString::from($s))
    };
}

macro_rules! vec2ColsModelRcRc {
    ($($s:expr),*) => {
        {
            let cols: VecModel<StandardListViewItem> = VecModel::default();
            $(
                cols.push(str2StandardListViewItem!($s));
            )*
            ModelRc::from(Rc::new(cols))
        }
    };
}

macro_rules! atomicStore {
    ($a:expr, $v:expr) => {
        $a.store($v, std::sync::atomic::Ordering::Relaxed);
    };
}

macro_rules! atomicLoad {
    ($a:expr) => {
        $a.load(std::sync::atomic::Ordering::Relaxed)
    };
}

fn populate_files_list(ui: &AppWindow) {
    let path = ui.get_path_input_value();
    let rows: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    match std::fs::read_dir(&path) {
        Ok(dir) => {
            for entry in dir {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() || path.is_file() {
                        let filename = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        let metadata = path.metadata().unwrap();
                        let size = metadata.len();
                        let modified: DateTime<Local> = metadata.modified().unwrap().into();
                        let created: DateTime<Local> = metadata.created().unwrap().into();

                        rows.push(vec2ColsModelRcRc!(
                            filename,
                            format_size(size, WINDOWS),
                            modified.format("%F %X").to_string(),
                            created.format("%F %X").to_string(),
                            match path.is_dir() {
                                true => "Directory",
                                false => "File",
                            },
                            "",
                            ""
                        ));

                        continue;
                    }
                }
            }
        }
        Err(e) => {
            rows.push(vec2ColsModelRcRc!(
                "Error",
                e.to_string(),
                "",
                "",
                "",
                "",
                ""
            ));
            ui.set_rows_data_property(rows.clone().into());
            return;
        }
    }

    ui.set_rows_data_property(rows.clone().into());
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let last_selected_files_table_index = AtomicI32::new(-1);
    let times_clicked = AtomicI32::new(0);
    let clicked_at = AtomicU64::new(0);

    ui.on_add_button_clicked({
        let ui_handle = ui.as_weak();
        move || {
            ui_handle
                .unwrap()
                .set_path_input_value("Hello, World!".into());
        }
    });

    ui.on_path_input_updated({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();

            populate_files_list(&ui);
        }
    });

    ui.on_files_table_row_changed({
        let ui_handle = ui.as_weak();

        move || {
            let last_click = atomicLoad!(clicked_at);
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64;
            atomicStore!(clicked_at, now);
            let diff = now - last_click;
            if diff >= 1 {
                return;
            }

            let ui = ui_handle.unwrap();
            let selected_index = ui.get_selected_files_table_index();

            let last_selected_index = atomicLoad!(last_selected_files_table_index);
            atomicStore!(last_selected_files_table_index, selected_index);
            dbg!(
                atomicLoad!(times_clicked),
                selected_index,
                last_selected_index
            );
            if last_selected_index == selected_index || selected_index == -1 {
                times_clicked.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if atomicLoad!(times_clicked) > 2 {
                    atomicStore!(times_clicked, 0);
                    return;
                }
            } else {
                atomicStore!(times_clicked, 0);
            }

            atomicStore!(last_selected_files_table_index, selected_index);
            if selected_index > -1 {
                if atomicLoad!(times_clicked) != 1 {
                    return;
                }
                let row = ui
                    .get_rows_data_property()
                    .row_data(selected_index as usize)
                    .unwrap();
                let filename = row.row_data(0).unwrap().text;
                let ftype = row.row_data(4).unwrap().text;
                let path = ui.get_path_input_value().to_string();

                match ftype.as_str() {
                    "Directory" => {
                        let path = std::path::Path::new(
                            path.as_str().trim_end_matches(std::path::MAIN_SEPARATOR),
                        );
                        let new_path = path.join(filename.to_string());
                        ui.set_path_input_value(new_path.to_string_lossy().to_string().into());
                        populate_files_list(&ui);
                    }
                    "File" => {
                        let path = std::path::Path::new(
                            path.as_str().trim_end_matches(std::path::MAIN_SEPARATOR),
                        );
                        let new_path = path.join(filename.to_string());
                        match open::that(new_path) {
                            Ok(_) => {}
                            Err(e) => {
                                ui.set_error_message(e.to_string().into());
                                ui.invoke_show_error_dialog();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    ui.on_up_directory({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            let path = ui.get_path_input_value();
            let path = std::path::Path::new(&path);
            let parent = path.parent().unwrap();
            ui.set_path_input_value(parent.to_string_lossy().to_string().into());
            populate_files_list(&ui);
        }
    });

    let home_folder = dirs::home_dir().unwrap().to_string_lossy().to_string();
    ui.set_path_input_value(home_folder.into());
    populate_files_list(&ui);

    ui.run()?;

    Ok(())
}
