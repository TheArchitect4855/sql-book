use postgres::{Client, Error, SimpleQueryMessage};

use crate::structs;

pub fn query(client: &mut Client, query: &str) -> Result<structs::Table, Error> {
	let query = client.simple_query(query)?;

	let mut cols = Vec::new();
	let mut rows = Vec::new();
	for row in query {
		let values = match row {
			SimpleQueryMessage::CommandComplete(v) => vec![ format!("Queried {} rows", v) ],
			SimpleQueryMessage::Row(row) => {
				if cols.len() == 0 {
					for col in row.columns() {
						cols.push(structs::Column {
							name: col.name().to_string(),
						})
					}
				}

				let mut values = Vec::new();
				for i in 0..row.len() {
					values.push(row.get(i).unwrap_or("[NULL]").to_string());
				}

				values
			},
			_ => vec![ String::from("Error: Type unimplemented") ],
		};

		rows.push(structs::Row {
			values: values.into_boxed_slice(),
		})
	}

	Ok(structs::Table {
		cols: cols.into_boxed_slice(),
		rows: rows.into_boxed_slice(),
	})
}
