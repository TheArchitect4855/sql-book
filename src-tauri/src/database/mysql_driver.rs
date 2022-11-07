use mysql::{prelude::Queryable, Value, Conn};

use crate::structs::{self, Table};

pub fn connect(uri: &str) -> Result<Conn, mysql::Error> {
	Conn::new(uri)
}

pub fn query(conn: &mut Conn, query: &str) -> Result<structs::Table, mysql::Error> {
	let rows = conn.query_iter(query)?;
	let cols = rows.columns();

	let mut rcols = Vec::with_capacity(cols.as_ref().len());
	for col in cols.as_ref() {
		rcols.push(structs::Column {
			name: col.name_str().to_string(),
		})
	}

	let mut rrows = Vec::new();
	for row in rows {
		let row = row?;
		let values: Vec<String> = row.unwrap()
			.into_iter()
			.map(display_value)
			.collect();

		rrows.push(structs::Row {
			values: values.into_boxed_slice(),
		})
	}

	Ok(Table {
		cols: rcols.into_boxed_slice(),
		rows: rrows.into_boxed_slice(),
	})
}

fn display_value(v: Value) -> String {
	let s = match v {
		Value::NULL => String::from("[NULL]"),
		Value::Bytes(v) => try_convert_str(v),
		Value::Int(v) => v.to_string(),
		Value::UInt(v) => v.to_string(),
		Value::Float(v) => v.to_string(),
		Value::Double(v) => v.to_string(),
		Value::Date(y, m, d, h, i, s, us) => format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}", y, m, d, h, i, s, us),
		Value::Time(neg, d, h, m, s, us) => format!("{}{:02}:{:02}:{:02}:{02}:{:02}.{:06}", match neg { true => "+", false => "-" }, d, h, m, s, us),
	};

	s
}

fn try_convert_str(bytes: Vec<u8>) -> String {
	match String::from_utf8(bytes.clone()) {
		Ok(v) => v,
		Err(_) => crate::util::to_hex(&bytes),
	}
}
