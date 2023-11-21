use std::fs;
use std::fs::File;
use std::path::Path;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    if cfg!(feature = "postgres") {
        println!("cargo:rustc-env=DATABASE_URL=postgres://user:pass@localhost/database");
    } else if cfg!(feature = "sqlite") {
        let db_path = "sqlite.db";
        println!("cargo:rustc-env=DATABASE_URL=sqlite://{}", db_path);

        let path = Path::new(db_path);
        if !path.exists() {
            let _ = File::create(path).expect("Could not create file");
        }

        let sql =
            fs::read_to_string("src/db/sql/1_setup.up.sql").expect("Unable to read the SQL file");
        let conn = Connection::open(db_path)?;
        conn.execute_batch(&sql)?;
    }

    Ok(())
}
