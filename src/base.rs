use base64::Engine;
use mysql_async::{
    params,
    prelude::{FromRow, FromValue, Query, Queryable, WithParams},
    Row,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Environment Error")]
    Env(#[from] std::env::VarError),
    #[error("ParseBool")]
    ParseBool(#[from] std::str::ParseBoolError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("MalformedRequest")]
    MalformedRequest,
    #[error("Database Error")]
    Database(#[from] mysql_async::Error),
    #[error("Email Error")]
    Lettre(#[from] lettre::error::Error),
    #[error("Email Transport Error")]
    LettreTransport(#[from] lettre::transport::smtp::Error),
    #[error("Base64 Error")]
    Base64(#[from] base64::DecodeError),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    // DB settings
    pub db_arch: String, // = 'mysql';
    pub db_name: String, // = 'database_name';
    pub db_host: String, // = 'database_host';
    pub db_user: String, // = 'database_user';
    pub db_pass: String, // = 'database_password';

    // Filesystem settings
    pub int_file_dir: String, // = "/home/my_username/public_html/file_exchange/files/";	// Internal directory where files reside
    pub ext_file_dir: String, // = "http://mydomain.com/file_exchange/files/";			// Weburl where files will be downloaded from

    // Link settings
    pub index_link: String, // = "http://mydomain.com/file_exchange/";					// Index url of your file exchange
    pub img_link: String, // = "http://mydomain.com/file_exchange/images/";              // Url to the images for your file exchange
    pub cat_link: String, // = "http://mydomain.com/file_exchange/category_images/";     // Url to the category images for your file exchange

    // Log settings
    pub do_log: bool,     // = false;
    pub log_file: String, // = "/home/my_username/logs/file_exchange/exchange.log";

    // Mail settings
    pub email_orig: String, // = "file_exchange@mydomain.com";							// E-mail address to send administrative e-mails from
    pub api_version: String, // api version
}
impl Config {
    pub(crate) async fn connect_db(&self) -> Result<mysql_async::Conn> {
        let opts = mysql_async::OptsBuilder::default()
            .ip_or_hostname(&self.db_host)
            .user(Some(&self.db_user))
            .pass(Some(&self.db_pass))
            .db_name(Some(&self.db_name));

        let conn_pool = mysql_async::Pool::new(opts);

        Ok(conn_pool.get_conn().await?)
    }
}

pub(crate) struct Base;
impl Base {
    pub(crate) async fn get_auth(
        config: std::sync::Arc<Config>,
        username: String,
        password: md5::Digest,
        ip: String,
    ) -> Result<usize> {
        let password = String::from_utf8_lossy(&password.to_vec()).to_string();

        let mut conn = config.connect_db().await?;

        let user = "SELECT * FROM LEX_USERS WHERE UPPER(USRNAME) = UPPER(:username) AND USRPASS = :password AND ISACTIVE = 'T'"
            .with(params! {
                "username" => username.to_uppercase(),
                password
            })
            .map(&mut conn, LEXUser::new).await?;

        if user.len() == 1 {
            "UPDATE LEX_USERS SET LASTIP = :ip, LASTLOGIN = :date, LOGINCNT = :count WHERE USRID = :usrid"
            .with(params! {
                ip,
                "date" => chrono::Utc::now().format("%Y%m%d%H%M%S").to_string(),
                "usrid" => user.first().unwrap().usrid
            }).ignore(&mut conn).await?;

            Ok(user.first().unwrap().usrid)
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub(crate) async fn is_auth(
        config: std::sync::Arc<Config>,
        username: String,
        password: md5::Digest,
        ip: String,
    ) -> Result<bool> {
        Ok(Base::get_auth(config, username, password, ip).await.is_ok())
    }

    pub(crate) async fn is_admin(config: std::sync::Arc<Config>, usrid: usize) -> Result<bool> {
        let mut conn = config.connect_db().await?;

        let admin: Vec<String> = "SELECT ISADMIN FROM LEX_USERS WHERE USRID = :usrid"
            .with(params! {
                usrid,
            })
            .map(&mut conn, |usrid| usrid)
            .await?;

        Ok(admin.first() == Some(&"T".to_string()))
    }
}

pub(crate) fn get_auth_from_headers(headers: warp::hyper::HeaderMap) -> (String, md5::Digest) {
    todo!()
}
