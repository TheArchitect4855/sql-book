use std::{sync::{Mutex, mpsc::{SyncSender, self, Receiver}}, thread, fs::File, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::structs::{self, Table};

mod mysql_driver;
mod postgres_driver;

enum Command {
	AddConn(ConnectionInfo),
	GetConns,
	PutConn(u16, ConnectionInfo),
	RemoveConn(u16),
	Query(u16, String),
}

enum CommandResult {
	Error(String),
	ConnId(u16),
	Connections(Box<[structs::ConnectionInfo]>),
	Connection(structs::ConnectionInfo),
	Query(structs::Table),
}

struct Connection {
	info: ConnectionInfo,
	driver: Option<ConnectionDriver>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConnectionInfo {
	name: String,
	uri: String,
	driver: String,
}

enum ConnectionDriver {
	MySql(mysql::Conn),
	Postgres(postgres::Client),
}

type CommandBundle = (
	Command,
	SyncSender<CommandResult>,
);

static INIT_LOCK: Mutex<bool> = Mutex::new(false);
static mut CMD_SEND: Option<SyncSender<CommandBundle>> = None;

pub fn init() {
	let mut lock = INIT_LOCK.lock()
		.expect("[DATABASE] Init lock poisoned");

	if *lock {
		panic!("[DATABASE] Init must only be called once");
	} else {
		*lock = true;
	}

	let (send, recv) = mpsc::sync_channel(8);

	unsafe {
		CMD_SEND = Some(send);
	}

	thread::spawn(|| run(recv));
	std::mem::drop(lock);
}

pub fn add_connection(info: ConnectionInfo) -> Result<u16, String> {
	let command = Command::AddConn(info);
	match do_command(command) {
		CommandResult::ConnId(v) => Ok(v),
		CommandResult::Error(e) => Err(e),
		_ => panic!()
	}
}

pub fn get_connections() -> Result<Box<[structs::ConnectionInfo]>, String> {
	let command = Command::GetConns;
	match do_command(command) {
		CommandResult::Connections(v) => Ok(v),
		CommandResult::Error(e) => Err(e),
		_ => panic!()
	}
}

pub fn edit_connection(id: u16, info: ConnectionInfo) -> Result<structs::ConnectionInfo, String> {
	let command = Command::PutConn(id, info);
	match do_command(command) {
		CommandResult::Connection(v) => Ok(v),
		CommandResult::Error(e) => Err(e),
		_ => panic!()
	}
}

pub fn remove_connection(id: u16) -> Result<Box<[structs::ConnectionInfo]>, String> {
	let command = Command::RemoveConn(id);
	match do_command(command) {
		CommandResult::Connections(v) => Ok(v),
		CommandResult::Error(e) => Err(e),
		_ => panic!()
	}
}

pub fn query(conn_id: u16, query: String) -> Result<structs::Table, String> {
	let command = Command::Query(conn_id, query);
	match do_command(command) {
		CommandResult::Query(v) => Ok(v),
		CommandResult::Error(e) => Err(e),
		_ => panic!()
	}
}

fn get_sender() -> SyncSender<CommandBundle> {
	unsafe {
		CMD_SEND
			.as_ref()
			.expect("[DATABASE] Not initialized")
			.clone()
	}
}

fn do_command(command: Command) -> CommandResult {
	let send = get_sender();
	let (o_send, o_recv) = mpsc::sync_channel(1);
	if let Err(e) = send.send((command, o_send)) {
		return CommandResult::Error(format!("Thread error: {}", e));
	}

	match o_recv.recv() {
		Ok(v) => v,
		Err(e) => return CommandResult::Error(format!("Thread Error: {}", e)),
	}
}

fn run(receiver: Receiver<CommandBundle>) {
	// Load connection info
	let mut connections: Vec<Connection> = load_connections()
		.iter()
		.map(|info| Connection {
			info: info.clone(),
			driver: None,
		}).collect();

	// Process commands
	loop {
		let Ok((command, ret)) = receiver.recv() else {
			println!("[DATABASE] Sending thread hung up");
			break;
		};

		let res = match command {
			Command::AddConn(info) => {
				match connect(&info) {
					Ok(driver) => {
						let id = connections.len() as u16;
						let conn = Connection {
							info,
							driver: Some(driver),
						};

						connections.push(conn);
						CommandResult::ConnId(id)
					},
					Err(e) => {
						CommandResult::Error(e)
					}
				}
			},
			Command::GetConns => CommandResult::Connections(map_conns(&connections)),
			Command::PutConn(id, info) => {
				match connect(&info) {
					Ok(driver) => {
						let res = structs::ConnectionInfo {
							name: info.name.clone(),
							host: info.get_host(),
							id,
						};

						connections[id as usize] = Connection {
							info,
							driver: Some(driver),
						};

						CommandResult::Connection(res)
					},
					Err(e) => CommandResult::Error(e),
				}
			},
			Command::RemoveConn(id) => {
				connections.remove(id as usize);
				CommandResult::Connections(map_conns(&connections))
			},
			Command::Query(conn_id, query) => {
				let conn = &mut connections[conn_id as usize];
				if let Some(driver) = &mut conn.driver {
					match do_query(driver, query) {
						Ok(v) => CommandResult::Query(v),
						Err(e) => CommandResult::Error(e),
					}
				} else {
					match connect(&conn.info) {
						Ok(driver) => {
							conn.driver = Some(driver);
							match do_query(conn.driver.as_mut().unwrap(), query) {
								Ok(v) => CommandResult::Query(v),
								Err(e) => CommandResult::Error(e),
							}
						},
						Err(e) => CommandResult::Error(e)
					}
				}
			},
		};

		if let Err(e) = ret.send(res) {
			eprintln!("[DATABASE] Send result failed: {}", e);
		}

		if let Err(e) = save_connections(&connections) {
			eprintln!("[DATABASE] Failed to save connections: {}", e);
		}
	}
}

fn connect(info: &ConnectionInfo) -> Result<ConnectionDriver, String> {
	match info.driver.as_str() {
		"mysql" => {
			mysql_driver::connect(&info.uri)
				.map(|v| ConnectionDriver::MySql(v))
				.map_err(|e| format!("Connection Error: {}", e))
		},
		"postgres" => {
			postgres::Client::connect(&info.uri, postgres::NoTls)
				.map(|v| ConnectionDriver::Postgres(v))
				.map_err(|e| format!("Connection Error: {}", e))
		}
		_ => Err(String::from("Unknown driver type")),
	}
}

fn map_conns(connections: &[Connection]) -> Box<[structs::ConnectionInfo]> {
	let mut res = Vec::with_capacity(connections.len());
	for (i, conn) in connections.iter().enumerate() {
		res.push(structs::ConnectionInfo {
			name: conn.info.name.clone(),
			host: conn.info.get_host(),
			id: i as u16,
		});
	}

	res.into_boxed_slice()
}

fn load_connections() -> Vec<ConnectionInfo> {
	let file = match File::open(get_conn_save_path()) {
		Ok(v) => v,
		Err(e) => {
			eprintln!("Failed to open connections.json: {}", e);
			return Vec::new();
		}
	};

	let res = match serde_json::from_reader(file) {
		Ok(v) => v,
		Err(e) => {
			eprintln!("Failed to parse connections.json: {}", e);
			return Vec::new();
		}
	};

	res
}

fn do_query(driver: &mut ConnectionDriver, query: String) -> Result<Table, String> {
	let upper = query.as_str().to_ascii_uppercase();
	let query = if upper.as_str().trim().starts_with("SELECT") && !upper.as_str().contains("LIMIT") {
		if query.as_str().trim().ends_with(";") {
			let q = query.as_str().trim();
			format!("{}\nLIMIT 10 -- SQL BOOK\n", &q[..q.len() - 1])
		} else {
			format!("{}\nLIMIT 10 -- SQL BOOK", query)
		}
	} else {
		query
	};

	println!("Performing query:\n{}", query);
	match driver {
		ConnectionDriver::MySql(conn) => mysql_driver::query(conn, &query)
			.map_err(|e| format!("Query Error: {}", e)),
		ConnectionDriver::Postgres(client) => postgres_driver::query(client, &query)
			.map_err(|e| format!("Query Error: {}", e)),
	}
}

fn save_connections(conns: &[Connection]) -> Result<(), String> {
	let file = File::create(get_conn_save_path())
		.map_err(|e| format!("IO Error: {}", e))?;

	let write: Vec<&ConnectionInfo> = conns.iter()
		.map(|v| &v.info)
		.collect();

	serde_json::to_writer(file, &write)
		.map_err(|e| e.to_string())
}

fn get_conn_save_path() -> PathBuf {
	let mut p = dirs::home_dir()
		.expect("No home dir");

	p.push(".sqlbook");
	if !p.exists() {
		std::fs::create_dir(&p)
			.expect("Failed to create save directory");
	}

	p.push("connections.json");
	p
}

impl ConnectionInfo {
	pub fn get_host(&self) -> String {
		let mut parts = self.uri.split("@");
		let url = parts.nth(1).unwrap_or("");
		let mut url_parts = url.split("/");
		let host = url_parts.nth(0).unwrap_or("");
		host.to_string()
	}
}
