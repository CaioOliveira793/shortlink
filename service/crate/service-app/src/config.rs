pub mod env {
    use once_cell::sync::Lazy;

    static ENV_VAR: Lazy<EnvVar> = Lazy::new(|| load_env());

    #[derive(Debug, Clone)]
    pub struct EnvVar {
        pub port: u16,
        pub token_key: String,
        pub surreal_host: String,
        pub surreal_port: u16,
        pub surreal_user: String,
        pub surreal_password: String,
    }

    macro_rules! get_env {
        ($env:literal) => {
            std::env::var($env).expect(concat!("Missing env var ", $env))
        };
    }

    fn load_env() -> EnvVar {
        let port: u16 = get_env!("PORT").parse().expect("Invalid PORT");
        let token_key = get_env!("TOKEN_KEY");
        let surreal_host = get_env!("SURREAL_HOST");
        let surreal_port = get_env!("SURREAL_PORT")
            .parse()
            .expect("Invalid SURREAL_PORT");
        let surreal_user = get_env!("SURREAL_USER");
        let surreal_password = get_env!("SURREAL_PASSWORD");

        EnvVar {
            port,
            token_key,
            surreal_host,
            surreal_port,
            surreal_user,
            surreal_password,
        }
    }

    pub fn get() -> &'static EnvVar {
        &ENV_VAR
    }
}

pub mod database {
    use surrealdb::{
        engine::{
            local::{Db, Mem},
            remote::ws::{Client, Ws},
        },
        opt::auth::Root,
        Surreal,
    };

    use super::env;

    pub async fn create_surrealdb_client() -> Surreal<Client> {
        let surreal_addr = format!("{}:{}", env::get().surreal_host, env::get().surreal_port);

        let db = Surreal::new::<Ws>(surreal_addr)
            .await
            .expect("failed to connect to surrealdb");
        db.signin(Root {
            username: &env::get().surreal_user,
            password: &env::get().surreal_password,
        })
        .await
        .expect("failed to signin to surrealdb");
        db.use_ns("shortlink")
            .use_db("shortlink_db")
            .await
            .expect("failed to use surreal database shortlink_db");
        db
    }

    #[allow(dead_code)]
    pub async fn create_surrealdb_memory_client() -> Surreal<Db> {
        let db = Surreal::new::<Mem>(())
            .await
            .expect("failed to create in memory surrealdb instance");

        db.use_ns("shortlink")
            .use_db("shortlink_db")
            .await
            .expect("failed to use surreal database shortlink_db");
        db
    }
}
