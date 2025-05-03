pub mod models{
    pub mod user;
    pub mod response;
}

pub mod handlers {
    pub mod user {
        pub mod user;
    }
}

pub mod usecases {
    pub mod user {
        pub mod user;
        pub use user::{
            find_user_by_id,
            create_user,
        };
    }
}

pub mod routes{
    pub mod route;
    pub use route::user_routes;
}