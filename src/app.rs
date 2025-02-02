use rusqlite::{Connection, Row};
use std::{fs, io};

const FILE_PATHS: [&str; 3] = [".db", ".sqlite3", ".db3"];

const SQL_TABLES: &str = "
    SELECT type,name,sql,tbl_name FROM main.sqlite_master;
";

#[derive(Debug, Default)]
pub struct Db {
    pub path: String,
    pub tables: Vec<Table>,
    pub views: Vec<Table>,
}

#[derive(Debug, Default)]
pub struct Table {
    pub name: String,
    pub sql: String,
}

#[derive(Debug, Default)]
pub struct App {
    pub current_db: Option<Db>,
}

impl App {
    /// Load at database at a given path
    pub fn load_db(&mut self, path: &str) -> Result<(), rusqlite::Error> {
        let con = Connection::open(path)?;
        let (tables, views) = get_tables(&con)?;

        self.current_db = Some(Db {
            path: path.to_string(),
            tables,
            views,
        });
        Ok(())
    }

    /// Select * from a given Table, Returns (Vec Column Names, Vec Row Data)
    pub fn select(
        &self,
        table: &Table,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), rusqlite::Error> {
        let sql = format!("SELECT * FROM {};", table.name);
        if let Some(db) = &self.current_db {
            let con = Connection::open(&db.path)?;
            let mut stmt = con.prepare(&sql)?;

            let num_of_columns = stmt.column_names().len();
            let data: Vec<Vec<String>> = stmt
                .query_map([], |row| map_row(num_of_columns, row))?
                .map(|x| (x.unwrap_or_default()))
                .collect();
            return Ok((
                stmt.column_names().iter().map(|x| x.to_string()).collect(),
                data,
            ));
        }
        Ok((Vec::default(), Vec::default()))
    }
}

fn map_row(num_of_columns: usize, row: &Row) -> Result<Vec<String>, rusqlite::Error> {
    let mut data: Vec<String> = Vec::default();
    for ind in 0..num_of_columns {
        match row.get_ref(ind) {
            Ok(column_ref) => match column_ref {
                rusqlite::types::ValueRef::Null => {
                    data.push("null".to_string());
                }
                rusqlite::types::ValueRef::Integer(v) => data.push(v.to_string()),
                rusqlite::types::ValueRef::Real(v) => data.push(v.to_string()),
                rusqlite::types::ValueRef::Text(s) => {
                    data.push(if let Ok(s) = String::from_utf8(s.to_vec()) {
                        s
                    } else {
                        "unreadable".to_string()
                    });
                }
                rusqlite::types::ValueRef::Blob(_) => data.push("Blob".to_string()),
            },
            Err(e) => return Err(e),
        }
    }
    Ok(data)
}

/// Returns (Vec Tables, Vec Views)
fn get_tables(con: &Connection) -> Result<(Vec<Table>, Vec<Table>), rusqlite::Error> {
    let mut stmt = con.prepare(SQL_TABLES)?;
    let mut tables: Vec<Table> = Vec::default();
    let mut views: Vec<Table> = Vec::default();
    let rows = stmt.query_map([], |row| {
        let type_id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let sql: Option<String> = row.get(2)?;
        let table_name: String = row.get(3)?;
        Ok((type_id, name, sql, table_name))
    })?;
    for (type_id, name, sql, _3) in rows.flatten() {
        if type_id == "table" {
            tables.push(Table {
                name,
                sql: sql.unwrap_or("".to_string()),
            });
        } else if type_id == "view" {
            views.push(Table {
                name,
                sql: sql.unwrap_or("".to_string()),
            });
        }
    }
    Ok((tables, views))
}

/// Returns list of files in given directory matching [FILE_PATHS]
pub fn load_files() -> io::Result<Vec<String>> {
    let mut files: Vec<String> = Vec::default();
    let paths = fs::read_dir("./")?;
    for p in paths {
        let path_as_str = p?.path().display().to_string();
        for endfix in FILE_PATHS {
            if path_as_str.ends_with(endfix) {
                files.push(path_as_str);
                break;
            }
        }
    }
    Ok(files)
}
