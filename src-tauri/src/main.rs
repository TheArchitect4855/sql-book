#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod database;
mod util;

mod structs;
use structs::*;

fn main() {
	database::init();
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			add_connection, get_connections, edit_connection, remove_connection,
			query,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
fn add_connection(info: database::ConnectionInfo) -> Result<u16, String> {
	database::add_connection(info)
}

#[tauri::command]
fn get_connections() -> Box<[ConnectionInfo]> {
	match database::get_connections() {
		Ok(v) => v,
		Err(e) => {
			eprintln!("Failed to get connections: {}", e);
			Box::new([])
		}
	}
}

#[tauri::command]
fn edit_connection(id: u16, info: database::ConnectionInfo) -> Result<ConnectionInfo, String> {
	database::edit_connection(id, info)
}

#[tauri::command]
fn remove_connection(id: u16) -> Result<Box<[ConnectionInfo]>, String> {
	database::remove_connection(id)
}

#[tauri::command]
fn query(conn_id: u16, query: String) -> Result<Table, String> {
	database::query(conn_id, query)
}
