

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {

        use crate::app::models::Person;
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {
            DB.connect::<Ws>("127.0.0.1:8000").await;
            DB.signin(Root {
                username: "root",
                password: "123hitesh"
            })
            .await;
            let _ = DB.use_ns("surreal").use_db("person").await;
        }

        pub async fn get_all_persons() -> Option<Vec<Person>> {
            open_db_connection().await;
            let get_all_person = DB.query("SELECT * FROM person ORDER BY joined_date DESC;").await;
            let _ = DB.invalidate().await;

            match get_all_person {
                Ok(mut res) => {
                    let found = res.take(0);
                    match found {
                        Ok(found_person) => Some(found_person),
                        Err(_) => None,
                    }
                },
                Err(_) => None,
            }
        }

        pub async fn add_person(new_person: Person) -> Option<Person> {
            open_db_connection().await;
            let results = DB.create(("person", new_person.uuid.clone()))
                .content(new_person)
                .await;
            let _ = DB.invalidate().await;

            // match results {
            //     Ok(created_person) => created_person,
            //     Err(_) => None
            // }
            match results {
                Ok(created_person) => created_person,
                Err(e) => {
                    println!("error in adding person: {:?}",e);
                    None
                }
            }

        }        
    }
}