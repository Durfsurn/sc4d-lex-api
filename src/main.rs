mod base;
mod category;
mod email;
mod lot;
mod routes;
mod search;
mod user;

use {base::*, user::*, lot::*, search::*, category::*};

use std::sync::Arc;

use futures::FutureExt;
use itertools::Itertools;
use warp::Filter;

const LISTEN_ADDRESS: &str = "0.0.0.0:0";

#[tokio::main]
async fn main() {
    // enable logging
    if std::env::var("do_log").unwrap_or_else(|_| "No envvar `do_log`.".into()) == "true" {
        simple_logging::log_to_file(
            std::env::var("log_file").unwrap_or_else(|_| "No envvar `logfile`.".into()),
            log::LevelFilter::Info,
        )
        .expect("Logging initialise failed.");
    }

    if let Err(e) = server().await {
        // if there is an error spin up a warp sever to display it gracefully to the user
        eprintln!("{e:?}");

        let err = e.to_string().lines().map(|l| format!("\t{l}")).join("\n");
        warp::serve(warp::any().map(move || format!("LEX Error:\n\t{err}")))
            .run(LISTEN_ADDRESS.parse::<std::net::SocketAddrV4>().unwrap())
            .await;
    }
}

async fn server() -> Result<()> {
    let config = Arc::new(Config {
        db_arch: std::env::var("db_arch")?,
        db_name: std::env::var("db_name")?,
        db_host: std::env::var("db_host")?,
        db_user: std::env::var("db_user")?,
        db_pass: std::env::var("db_pass")?,
        int_file_dir: std::env::var("int_file_dir")?,
        ext_file_dir: std::env::var("ext_file_dir")?,
        index_link: std::env::var("index_link")?,
        img_link: std::env::var("img_link")?,
        cat_link: std::env::var("cat_link")?,
        do_log: std::env::var("do_log")?.parse()?,
        log_file: std::env::var("log_file")?,
        email_orig: std::env::var("email_orig")?,
        api_version: std::env::var("api_version")?,
    });
    let with_config = |arc_config: Arc<Config>| warp::any().map(move || arc_config.clone());

    let get_endpoints = warp::any()
        .and_then(|| routes::get_endpoints().map(handle_application_error))
        .boxed();
    let get_version = warp::get()
        .and(warp::path("version"))
        .and(with_config(config.clone()))
        .and_then(|config| routes::get_version(config).map(handle_application_error))
        .boxed();
    let get_user = warp::get()
        .and(warp::path!("user"))
        .and(with_config(config.clone()))
        .and(warp::header::headers_cloned())
        .and(warp::filters::addr::remote())
        .and_then(|config, headers, remote| {
            routes::get_user(config, headers, remote).map(handle_application_error)
        })
        .boxed();
    let adm_get_all = warp::get()
        .and(warp::path!("user/all"))
        .and(with_config(config.clone()))
        .and(warp::header::headers_cloned())
        .and(warp::filters::addr::remote())
        .and(warp::query())
        .and_then(|config, headers, remote, query| {
            routes::adm_get_all(config, headers, remote, query).map(handle_application_error)
        })
        .boxed();
    let adm_get_user = warp::get()
        .and(warp::path!("user" / usize))
        .and(with_config(config.clone()))
        .and(warp::header::headers_cloned())
        .and(warp::filters::addr::remote())
        .and_then(|usrid, config, headers, remote| {
            routes::adm_get_user(config, headers, remote, usrid).map(handle_application_error)
        })
        .boxed();
    let get_download_history = warp::get()
        .and(warp::path!("user/download-history"))
        .and(with_config(config.clone()))
        .and(warp::header::headers_cloned())
        .and(warp::filters::addr::remote())
        .and_then(|config, headers, remote| {
            routes::get_download_history(config, headers, remote).map(handle_application_error)
        })
        .boxed();
    let get_download_list = warp::get()
        .and(warp::path!("user/download-list"))
        .and(with_config(config.clone()))
        .and(warp::header::headers_cloned())
        .and(warp::filters::addr::remote())
        .and_then(|config, headers, remote| {
            routes::get_download_list(config, headers, remote).map(handle_application_error)
        })
        .boxed();
    let post_register_user = warp::post()
        .and(warp::path!("user/register"))
        .and(with_config(config.clone()))
        .and(warp::header::headers_cloned())
        .and(warp::filters::addr::remote())
        .and(warp::query())
        .and_then(|config, headers, remote, query| {
            routes::post_register_user(config, headers, remote, query).map(handle_application_error)
        })
        .boxed();
    let get_activate_user = warp::get()
        .and(warp::path!("user/activate"))
        .and(with_config(config.clone()))
        .and(warp::query())
        .and_then(|config, query| {
            routes::get_activate_user(config, query).map(handle_application_error)
        })
        .boxed();
    let get_all_lots = warp::get()
        .and(warp::path!("lot/all"))
        .and_then(|| routes::get_all_lots().map(handle_application_error))
        .boxed();
    let get_lot_http = warp::get()
        .and(warp::path!("lot" / String))
        .and_then(|lot| routes::get_lot_http(lot).map(handle_application_error))
        .boxed();
    let get_download = warp::get()
        .and(warp::path!("lot" / String / "download"))
        .and_then(|lot| routes::get_download(lot).map(handle_application_error))
        .boxed();
    let do_download_list = warp::get()
        .and(warp::path!("lot" / String / "download-list"))
        .and_then(|lot| routes::do_download_list(lot).map(handle_application_error))
        .boxed();
    let bulk_download = warp::get()
        .and(warp::path!("lot" / String / "bulk-dependency"))
        .and_then(|lot| routes::bulk_download(lot).map(handle_application_error))
        .boxed();
    let delete_download_list = warp::delete()
        .and(warp::path!("lot" / String / "download-list"))
        .and_then(|lot| routes::delete_download_list(lot).map(handle_application_error))
        .boxed();
    let get_comment_http = warp::get()
        .and(warp::path!("lot" / String / "comment"))
        .and_then(|lot| routes::get_comment_http(lot).map(handle_application_error))
        .boxed();
    let post_comment = warp::post()
        .and(warp::path!("lot" / String / "comment"))
        .and_then(|lot| routes::post_comment(lot).map(handle_application_error))
        .boxed();
    let get_vote_http = warp::get()
        .and(warp::path!("lot" / String / "vote"))
        .and_then(|lot| routes::get_vote_http(lot).map(handle_application_error))
        .boxed();
    let get_lot_dependency = warp::get()
        .and(warp::path!("lot" / String / "dependency"))
        .and_then(|lot| routes::get_lot_dependency(lot).map(handle_application_error))
        .boxed();
    let get_dependency_string = warp::get()
        .and(warp::path!("lot" / String / "dependency-string"))
        .and_then(|lot| routes::get_dependency_string(lot).map(handle_application_error))
        .boxed();
    let update_dependency_string = warp::put()
        .and(warp::path!("lot" / String / "dependency-string"))
        .and_then(|lot| routes::update_dependency_string(lot).map(handle_application_error))
        .boxed();
    let do_search = warp::get()
        .and(warp::path!("search"))
        .and(with_config(config.clone()))
        .and(warp::header::headers_cloned())
        .and(warp::filters::addr::remote())
        .and(warp::query())
        .and_then(|config, headers, remote, query| routes::do_search(config, headers, remote, query).map(handle_application_error))
        .boxed();
    let get_broad_category = warp::get()
        .and(warp::path!("category/broad-category"))
        .and(with_config(config.clone()))
        .and_then(|config| routes::get_broad_category(config).map(handle_application_error))
        .boxed();
    let get_lex_category = warp::get()
        .and(warp::path!("category/lex-category"))
        .and(with_config(config.clone()))
        .and_then(|config| routes::get_lex_category(config).map(handle_application_error))
        .boxed();
    let get_lex_type = warp::get()
        .and(warp::path!("category/lex-type"))
        .and(with_config(config.clone()))
        .and_then(|config| routes::get_lex_type(config).map(handle_application_error))
        .boxed();
    let get_group = warp::get()
        .and(warp::path!("category/group"))
        .and(with_config(config.clone()))
        .and_then(|config| routes::get_group(config).map(handle_application_error))
        .boxed();
    let get_author = warp::get()
        .and(warp::path!("category/author"))
        .and(with_config(config.clone()))
        .and_then(|config| routes::get_author(config).map(handle_application_error))
        .boxed();
    let get_all_categories = warp::get()
        .and(warp::path!("category/all"))
        .and(with_config(config.clone()))
        .and_then(|config| routes::get_all_categories(config).map(handle_application_error))
        .boxed();

    let all_routes = warp::any()
        .and(
            warp::path("api").and(
                get_version
                    .or(get_user)
                    .or(adm_get_all)
                    .or(adm_get_user)
                    .or(get_download_history)
                    .or(get_download_list)
                    .or(post_register_user)
                    .or(get_activate_user)
                    .or(get_all_lots)
                    .or(get_lot_http)
                    .or(get_download)
                    .or(do_download_list)
                    .or(bulk_download)
                    .or(delete_download_list)
                    .or(get_comment_http)
                    .or(post_comment)
                    .or(get_vote_http)
                    .or(get_lot_dependency)
                    .or(get_dependency_string)
                    .or(update_dependency_string)
                    .or(do_search)
                    .or(get_broad_category)
                    .or(get_lex_category)
                    .or(get_lex_type)
                    .or(get_group)
                    .or(get_author)
                    .or(get_all_categories)
                    .or(get_endpoints),
            ),
        )
        .with(warp::log("server"));
    Ok(())
}

pub fn handle_application_error<'a>(
    result: Result<impl warp::Reply + 'a>,
) -> std::result::Result<Box<dyn warp::Reply + 'a>, std::convert::Infallible> {
    match result {
        Err(e) => {
            log::warn!("Request application error: {:#?}", e.to_string());
            // eprintln!("Request application error: {:#?}", e.to_string());
            match e {
                Error::Forbidden => {
                    let err = warp::reply::with_status(
                        warp::reply::json(&e.to_string()),
                        warp::http::StatusCode::FORBIDDEN,
                    );
                    Ok(Box::new(err))
                }
                Error::Unauthorized => {
                    let err = warp::reply::with_status(
                        warp::reply::json(&e.to_string()),
                        warp::http::StatusCode::UNAUTHORIZED,
                    );
                    Ok(Box::new(err))
                }
                Error::MalformedRequest => {
                    let err = warp::reply::with_status(
                        warp::reply::json(&e.to_string()),
                        warp::http::StatusCode::BAD_REQUEST,
                    );
                    Ok(Box::new(err))
                }
                _ => {
                    let err = warp::reply::with_status(
                        warp::reply::json(&e.to_string()),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    );
                    Ok(Box::new(err))
                }
            }
        }
        Ok(o) => Ok(Box::new(o)),
    }
}
