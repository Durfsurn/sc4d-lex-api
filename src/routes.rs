use crate::*;

pub(crate) async fn get_endpoints() -> Result<impl warp::Reply> {
    let response = serde_json::json!({
        "basic": {
            "/": "(GET) retrieves all endpoints for this API",
            "/version": "(GET) retrieves the current version of this API"
        },
        "user": {
            "/user": "(GET) retrieves profile information for the user",
            "/user/download-history" : "(GET) retrieves download history for the user",
            "/user/download-list" : "(GET) retrieves download list for the user",
            "/user/register" : "(POST) registers a new user for the LEX",
            "/user/activate" : "(GET) activates the registration for a LEX user"
        },
        "lot": {
            "/lot/all" : "(GET) retrieves a list of all lots",
            "/lot/:lotid" : "(GET) retrieves information about the lot with the supplied ID"
        },
        "search": {
            "/search" : "(GET) retrieves search results"
        },
        "interaction": {
            "/lot/:lotid/download" : "(POST) retrieves a download link for the lot with the supplied ID - also adds it to download history",
            "/lot/:lotid/download-list" : "(POST) adds the lot with the supplied ID to the download-later list"
        }
    });

    Ok(warp::reply::json(&response))
}
pub(crate) async fn get_version(config: Arc<Config>) -> Result<impl warp::Reply> {
    let response = serde_json::json!({
        "version": config.api_version,
        "type": "public"
    });

    Ok(warp::reply::json(&response))
}
pub(crate) async fn get_user(
    config: Arc<Config>,
    headers: warp::hyper::HeaderMap,
    remote: Option<std::net::SocketAddr>,
) -> Result<impl warp::Reply> {
    let (username, password) = get_auth_from_headers(headers);
    let ip = remote.ok_or(Error::Forbidden)?.to_string();

    LEXUser::get_user(config, username, password, ip, None).await
}
pub(crate) async fn adm_get_all(
    config: Arc<Config>,
    headers: warp::hyper::HeaderMap,
    remote: Option<std::net::SocketAddr>,
    query: serde_json::Value,
) -> Result<impl warp::Reply> {
    let (username, password) = get_auth_from_headers(headers);
    let ip = remote.ok_or(Error::Forbidden)?.to_string();

    LEXUser::adm_get_all(
        config,
        username,
        password,
        ip,
        query
            .get("rows_offset")
            .and_then(|q| q.as_u64())
            .ok_or(Error::MalformedRequest)?,
        query
            .get("rows_count")
            .and_then(|q| q.as_u64())
            .ok_or(Error::MalformedRequest)?,
        query
            .get("concise")
            .and_then(|q| q.as_bool())
            .ok_or(Error::MalformedRequest)?,
    )
    .await
}
pub(crate) async fn adm_get_user(
    config: Arc<Config>,
    headers: warp::hyper::HeaderMap,
    remote: Option<std::net::SocketAddr>,
    usrid: usize,
) -> Result<impl warp::Reply> {
    let (username, password) = get_auth_from_headers(headers);
    let ip = remote.ok_or(Error::Forbidden)?.to_string();

    LEXUser::adm_get_user(config, username, password, ip, usrid).await
}
pub(crate) async fn get_download_history(
    config: Arc<Config>,
    headers: warp::hyper::HeaderMap,
    remote: Option<std::net::SocketAddr>,
) -> Result<impl warp::Reply> {
    let (username, password) = get_auth_from_headers(headers);
    let ip = remote.ok_or(Error::Forbidden)?.to_string();

    LEXUser::get_download_history(config, username, password, ip).await
}
pub(crate) async fn get_download_list(
    config: Arc<Config>,
    headers: warp::hyper::HeaderMap,
    remote: Option<std::net::SocketAddr>,
) -> Result<impl warp::Reply> {
    let (username, password) = get_auth_from_headers(headers);
    let ip = remote.ok_or(Error::Forbidden)?.to_string();

    LEXUser::get_download_list(config, username, password, ip).await
}
pub(crate) async fn post_register_user(
    config: Arc<Config>,
    headers: warp::hyper::HeaderMap,
    remote: Option<std::net::SocketAddr>,
    query: serde_json::Value,
) -> Result<impl warp::Reply> {
    let ip = remote.ok_or(Error::Forbidden)?.to_string();

    LEXUser::register_user(
        query
            .get("username")
            .and_then(|q| q.as_str())
            .map(|q| q.to_string())
            .ok_or(Error::MalformedRequest)?,
        query
            .get("password_1")
            .and_then(|q| q.as_str())
            .map(|q| q.to_string())
            .ok_or(Error::MalformedRequest)?,
        query
            .get("password_2")
            .and_then(|q| q.as_str())
            .map(|q| q.to_string())
            .ok_or(Error::MalformedRequest)?,
        query
            .get("email")
            .and_then(|q| q.as_str())
            .map(|q| q.to_string())
            .ok_or(Error::MalformedRequest)?,
        query
            .get("fullname")
            .and_then(|q| q.as_str())
            .map(|q| q.to_string())
            .ok_or(Error::MalformedRequest)?,
        config,
        ip,
    )
    .await
}
pub(crate) async fn get_activate_user(
    config: Arc<Config>,
    query: serde_json::Value,
) -> Result<impl warp::Reply> {
    LEXUser::activate_user(
        config,
        query
            .get("activation_key")
            .and_then(|q| q.as_str())
            .map(|q| q.to_string())
            .ok_or(Error::MalformedRequest)?,
    )
    .await
}
pub(crate) async fn get_all_lots() -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn get_lot_http(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn get_download(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn do_download_list(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn bulk_download(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn delete_download_list(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn get_comment_http(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn post_comment(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn get_vote_http(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn get_lot_dependency(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn get_dependency_string(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn update_dependency_string(lot: String) -> Result<impl warp::Reply> {
    Ok(warp::reply())
}
pub(crate) async fn do_search(
    config: Arc<Config>,
    headers: warp::hyper::HeaderMap,
    remote: Option<std::net::SocketAddr>,
    query: SearchParams,
) -> Result<impl warp::Reply> {
    let (username, password) = get_auth_from_headers(headers);
    let ip = remote.ok_or(Error::Forbidden)?.to_string();

    Search::do_search(config, username, password, ip, query).await
}
pub(crate) async fn get_broad_category(config: Arc<Config>) -> Result<impl warp::Reply> {
    Ok(warp::reply::json(
        &Category::get_broad_category(config).await?,
    ))
}
pub(crate) async fn get_lex_category(config: Arc<Config>) -> Result<impl warp::Reply> {
    Ok(warp::reply::json(&Category::get_lex_category(config).await?))
}
pub(crate) async fn get_lex_type(config: Arc<Config>) -> Result<impl warp::Reply> {
    Ok(warp::reply::json(&Category::get_lex_type(config).await?))
}
pub(crate) async fn get_group(config: Arc<Config>) -> Result<impl warp::Reply> {
    Ok(warp::reply::json(&Category::get_group(config).await?))
}
pub(crate) async fn get_author(config: Arc<Config>) -> Result<impl warp::Reply> {
    Ok(warp::reply::json(&Category::get_author(config).await?))
}
pub(crate) async fn get_all_categories(config: Arc<Config>) -> Result<impl warp::Reply> {
    Category::get_all(config).await
}
