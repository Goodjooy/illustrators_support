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
    ($name:ident,$base:literal,$($routes:ident),*) => {
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
