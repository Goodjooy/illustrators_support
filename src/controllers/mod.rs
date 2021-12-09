use rocket::serde::{json::Json, DeserializeOwned};

pub mod admin;
pub mod file_server;
pub mod illustrator;
pub mod user;

pub trait Controller {
    fn routes() -> Vec<rocket::Route> {
        vec![]
    }
    fn base<'s>() -> &'s str {
        "/"
    }
}
#[macro_export]
macro_rules! generate_controller {
    ($name:ident,$base:literal,$($routes:path),*) => {
        pub struct $name;
        impl crate::controllers::Controller for $name  {
            fn routes()->Vec<rocket::Route>{
                rocket::routes![
                    $(
                        $routes
                    ),*
                ]
            }

            fn base<'s>()->&'s str{
                $base
            }
        }
    };
}

#[macro_export]
macro_rules! no_auth_handle {
    ($method:path,$n:ident,$au:ident,$path:literal) => {
        #[$method("/<path..>", rank = 4)]
        fn $n(path: std::path::PathBuf) -> crate::data_containers::r_result::RResult<String> {
            crate::data_containers::r_result::RResult::status_err(
                rocket::http::Status::Unauthorized,
                format!(
                    "`{}` AUTH Need for `/{}/{}`",
                    stringify!($au),
                    $path,
                    path.to_string_lossy()
                ),
            )
        }
    };
}

fn into_entity<T: DeserializeOwned>(input: Json<serde_json::Value>) -> Result<T, String> {
    match serde_json::from_value::<T>(input.0) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Json Error: {}", e)),
    }
}
