use rusqlite::{params, Connection};
use std::collections::HashMap;

pub struct Database {
    rows: std::vec::IntoIter<HashMap<String, String>>,
}

impl Database {
    pub fn fetch(conn: &Connection, query: &str) -> Self {
        let mut stml = conn.prepare(query).unwrap();
        let column_names: Vec<String> =
            stml.column_names().into_iter().map(|s| s.to_string()).collect();
        let rows = stml
            .query_map(params![], |row| {
                let mut map = HashMap::new();
                for (i, col) in column_names.iter().enumerate() {
                    let value = row.get::<_, rusqlite::types::Value>(i)?;
                    /*
                     * 現在全部Stringで登録される。
                     */
                    let value_str = match value {
                        rusqlite::types::Value::Null => "NULL".to_string(),
                        rusqlite::types::Value::Integer(i) => i.to_string(),
                        rusqlite::types::Value::Real(f) => f.to_string(),
                        rusqlite::types::Value::Text(s) => s,
                        rusqlite::types::Value::Blob(_) => "BLOB".to_string(),
                    };
                    map.insert(col.to_string(), value_str);
                }
                Ok(map)
            })
            .unwrap()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
            .into_iter();
        Self { rows }
    }
}

impl Iterator for Database {
    type Item = HashMap<String, String>; 
    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next()
    }
}


pub fn open(dbpath: &str) -> Result<Connection, rusqlite::Error> {
    Ok(Connection::open(&dbpath)?)
}

pub fn open_in_memory(dbpath: &str) -> Result<Connection, rusqlite::Error> {
    Ok(Connection::open_in_memory(&dbpath)?)
}

/*
use database::{open, Database};

fn demo {
    let mut con = open("_DATA/test.db").unwrap();
    let db = Database::fetch(&con, "select * from test_table");
    for row in db {
        println!("> {}({})", row["name"], row["age"]);
    }
    
    let trans = con.transaction().unwrap();
    for count in 0..100 {
        let sql = format!("insert into test_table (name, age) values ('Bob-{}', {})", count, count);
        trans.execute(&sql, []).unwrap();
    }
    trans.commit().unwrap();

    //トランザクションは目的ごとに作り直す。
    let trans = con.transaction().unwrap(); 
    let sql = "create table if not exists test (x integer, y integer)";
    trans.execute(&sql, []).unwrap();
    trans.commit().unwrap();

}

*/
