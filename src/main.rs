use crate::db::DB;

mod db;
mod models;

fn main() {
    let database_url = "mongodb://localhost/test__rocket_mongodb".to_string();
    let db = DB::new(database_url).unwrap();

    db.insert_data(String::from("n1"), 1, vec![String::from("a1"), String::from("a2")]).unwrap();
}
