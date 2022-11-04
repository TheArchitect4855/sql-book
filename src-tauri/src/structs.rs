use serde::Serialize;

#[derive(Serialize)]
pub struct ConnectionInfo {
	pub name: String,
	pub host: String,
	pub id: u16,
}

#[derive(Serialize)]
pub struct Table {
	pub cols: Box<[Column]>,
	pub rows: Box<[Row]>,
}

#[derive(Serialize)]
pub struct Column {
	pub name: String,
}

#[derive(Serialize)]
pub struct Row {
	pub values: Box<[String]>,
}
