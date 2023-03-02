use std::path::PathBuf;

use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};

fn get_static_path_for_uri(path: &str) -> &'static str {
    if path.starts_with("/game/") {
        return "game/[gameid].html";
    }

    match path {
        "/community" => "community.html",
        "/games" => "games.html",
        "/profile" => "profile.html",
        _ => "404.html",
    }
}

pub fn static_file_service(static_path: PathBuf) -> Files {
    Files::new("/", &static_path)
        .index_file("index.html")
        .default_handler(fn_service(move |req: ServiceRequest| {
            // annoying lifetime juggling for this async closure
            let inner_static_path = static_path.clone();
            async move {
                let (req, _) = req.into_parts();
                let path = get_static_path_for_uri(req.path());
                let file = NamedFile::open_async(inner_static_path.join(path)).await?;
                let res = file.into_response(&req);
                Ok(ServiceResponse::new(req, res))
            }
        }))
}
