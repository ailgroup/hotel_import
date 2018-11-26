extern crate postgres;
use postgres::{Connection, TlsMode};
use std::env;

//#[allow(dead_code)]
pub struct DBConn {
    pub conn: std::result::Result<postgres::Connection, postgres::Error>,
}

// related functions
//#[allow(dead_code)]
impl DBConn {
    pub fn make_connection_string(
        user: String,
        host: String,
        dbname: String,
        port: u32,
        need_pass: bool,
    ) -> Result<String, std::env::VarError> {
        let url = if need_pass {
            let pwrd = match env::var("HOTEL_IMPORT_DB_PASSWORD") {
                Ok(v) => v,
                Err(err) => {
                    return Err(err);
                }
            };
            format!("postgres://{}:{}@{}:{}/{}", user, pwrd, host, port, dbname)
        } else {
            format!("postgres://{}@{}:{}/{}", user, host, port, dbname)
        };

        Ok(url)
    }

    pub fn new(conn_string: String) -> DBConn {
        DBConn {
            conn: Connection::connect(conn_string, TlsMode::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_db_conn_string(pass: bool) -> String {
        let (n, h, d, p) = (
            String::from("jbowles"),
            String::from("localhost"),
            String::from("hotel_import"),
            5432,
        );

        return match DBConn::make_connection_string(n, h, d, p, pass) {
            Ok(v) => v,
            Err(err) => err.to_string(),
        };
    }

    #[test]
    fn test_make_connection_string() {
        let cs = mock_db_conn_string(true);
        assert_eq!(cs, "environment variable not found");

        env::set_var("HOTEL_IMPORT_DB_PASSWORD", "password");
        let cs = mock_db_conn_string(true);
        assert_eq!(
            cs,
            "postgres://jbowles:password@localhost:5432/hotel_import"
        );

        let cs = mock_db_conn_string(false);
        assert_eq!(cs, "postgres://jbowles@localhost:5432/hotel_import");
    }
}
