
pub mod store {
    use chrono::NaiveDate;
    use sqlite::State;

    use crate::Todo;


    static TODO_TABLE_NAME: &str = "todos";

    pub struct StoreSesssion {
        pub connection: sqlite::Connection,
    }

    impl StoreSesssion {
        pub fn open(path: &str) -> StoreSesssion{
            let connection = match sqlite::open(path) {
                Ok(connection) => connection,
                Err(msg) => panic!("Cannot create new db `{}` : {}", path, msg)
            };

            let session = StoreSesssion {
                connection,
            };
            session.setup_tables();
            session
        }

        pub fn setup_tables(&self) {
            self.connection.execute(
                format!("CREATE TABLE IF NOT EXISTS {} (title TEXT, date Date);", TODO_TABLE_NAME)
            ).unwrap();
        }

        pub fn add_todo(&self, title: &String, date: &NaiveDate) {
            let date_str = date.format("%Y-%m-%d").to_string();
            self.connection.execute(
                format!("INSERT INTO todos VALUES('{}', '{}');", title, date_str)
            ).unwrap();
        }

        pub fn fetch_all_todos(&self) -> Vec<Todo> {
            let mut statement = self.connection.prepare(
                format!("SELECT rowid, title, date FROM todos ORDER BY date")
            ).unwrap();

            let mut todos: Vec<Todo> = Vec::new();

            loop {
                match statement.next().unwrap() {
                    State::Done => break,
                    State::Row => {
                        let id = statement.read::<i64, _>(0).unwrap() as usize;
                        let title: String = statement.read(1).unwrap();
                        let date_str: String = statement.read(2).unwrap();

                        todos.push(
                            Todo {
                                id, title,
                                date:NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap()
                            }
                        );
                    }
                }
            }

            return todos;
        }

        pub fn delete_todo_by_id(&self, idx: usize) {
            self.connection.execute(
                format!("DELETE FROM todos WHERE rowid='{}';", idx)
            ).unwrap();
        }

    }
}