use rusqlite::{Connection, Result};

struct MyBookmark {
    name: String,
    url: String,
}

fn initialize_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            name TEXT NOT NULL,
            url TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

fn add_new_bookmark(conn: &Connection, name: &str, url: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO bookmarks (name, url) VALUES (?1, ?2)",
        [name, url],
    )?;
    println!("Semn de carte adăugat cu succes!");
    Ok(())
}

fn search_bookmarks(conn: &Connection, search_pattern: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name, url FROM bookmarks WHERE name LIKE ?1")?;

    let bookmarks_iter = stmt.query_map([format!("%{}%", search_pattern)], |row| {
        Ok(MyBookmark {
            name: row.get("name")?,
            url: row.get("url")?,
        })
    })?;

    for bookmark in bookmarks_iter {
        let bookmark = bookmark?;
        println!("name: {}, url: {}", bookmark.name, bookmark.url);
    }

    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("bookmarks.db")?;

    initialize_table(&conn)?;

    let command_line_args: Vec<String> = std::env::args().collect();

    if command_line_args.len() >= 3 && command_line_args[1] == "bm" {
        match command_line_args[2].as_str() {
            "add" => {
                if command_line_args.len() == 5 {
                    let name = &command_line_args[3];
                    let url = &command_line_args[4];
                    add_new_bookmark(&conn, name, url)?;
                } else {
                    eprintln!("Utilizare: bm add <name> <url>");
                }
            }
            "search" => {
                if command_line_args.len() == 4 {
                    let search_pattern = &command_line_args[3];
                    search_bookmarks(&conn, search_pattern)?;
                } else {
                    eprintln!("Utilizare: bm search <name>");
                }
            }
            _ => {
                eprintln!("Utilizare: bm <add,search> [name] [url]");
            }
        }
    } else {
        eprintln!("Utilizare: bm <add,search> [name] [url]");
    }

    Ok(())
}
