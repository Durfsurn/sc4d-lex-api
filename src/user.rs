use base64::Engine;
use mysql_async::{
    params,
    prelude::{FromRow, FromValue, Query, Queryable, WithParams},
    Row,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct LEXUser {
    pub(crate) usrid: usize,
    pub(crate) email_address: String,
    pub(crate) username: String,
    pub(crate) full_name: String,
    pub(crate) login_count: usize,
    pub(crate) registered: chrono::NaiveDateTime,
    pub(crate) last_login: chrono::NaiveDateTime,
    pub(crate) is_active: bool,
    pub(crate) usrlvl: usize,
    pub(crate) donator: bool,
    pub(crate) rater: bool,
    pub(crate) uploader: bool,
    pub(crate) author: bool,
    pub(crate) is_admin: bool,
}
impl LEXUser {
    pub(crate) fn new(row: Row) -> Self {
        let isactive: String = FromValue::from_value(row["isactive"].clone());
        let usrlvl: String = FromValue::from_value(row["usrlvl"].clone());
        LEXUser {
            usrid: FromValue::from_value(row["usrid"].clone()),
            email_address: FromValue::from_value(row["fullname"].clone()),
            username: FromValue::from_value(row["usrname"].clone()),
            full_name: FromValue::from_value(row["dateon"].clone()),
            login_count: FromValue::from_value(row["lastlogin"].clone()),
            registered: chrono::NaiveDateTime::parse_from_str(&isactive, "%Y%m%d").unwrap(),
            last_login: chrono::NaiveDateTime::parse_from_str(&usrlvl, "%Y%m%d%H%M%S").unwrap(),
            is_active: FromValue::from_value(row["emailadddr"].clone()),
            usrlvl: FromValue::from_value(row["logincnt"].clone()),
            donator: FromValue::from_value(row["donator"].clone()),
            rater: FromValue::from_value(row["rater"].clone()),
            uploader: FromValue::from_value(row["uploader"].clone()),
            author: FromValue::from_value(row["author"].clone()),
            is_admin: FromValue::from_value(row["isadmin"].clone()),
        }
    }
    pub(crate) async fn check_register(
        username: String,
        password_1: String,
        password_2: String,
        email: String,
        fullname: String,
        config: Arc<Config>,
        ip: String,
    ) -> Result<bool> {
        let mut conn = config.connect_db().await?;

        if password_1 == password_2 {
            let user =
                "SELECT * FROM LEX_USERS WHERE UPPER(USRNAME) = :tun OR UPPER(EMAILADDDR) = :tem"
                    .with(params! {
                        "tun" => &username.to_uppercase(),
                        "tem" => &email.to_uppercase(),
                    })
                    .map(&mut conn, LEXUser::new)
                    .await?;

            if user.len() == 1 {
                let ban_list = "SELECT * FROM LEX_IPBANS WHERE REGIP LIKE :ip1 OR LASTIP LIKE :ip2"
                    .with(params! {
                        "ip1" => &ip,
                        "ip2" => ip,
                    })
                    .map(&mut conn, LEXUser::new)
                    .await?;
                // if user is in the ban list return forbidden
                if ban_list.len() == 1 {
                    return Err(Error::Forbidden);
                } else {
                    return Ok(true);
                }
            }
        }
        Err(Error::Forbidden)
    }
    pub(crate) async fn register_user(
        username: String,
        password_1: String,
        password_2: String,
        email: String,
        fullname: String,
        config: Arc<Config>,
        ip: String,
    ) -> Result<impl warp::Reply> {
        let hashed_password = md5::compute(&password_1);
        let mut conn = config.connect_db().await?;

        LEXUser::check_register(
            username.clone(), password_1, password_2, email.clone(), fullname.clone(), config.clone(), ip.clone(),
        )
        .await?;

        "INSERT INTO LEX_USERS (FULLNAME,USRNAME,USRPASS,DATE ON,EMAILADDDR,ISACTIVE,REGIP)
            VALUES (:fullname, :username, :pass, :now, :email, 'P', :regip)"
            .with(params! {
                "fullname" => &fullname,
                "username" => &username,
                "pass" => String::from_utf8_lossy(&hashed_password.to_vec()).to_string(),
                "now" => chrono::Utc::now().date_naive().to_string(),
                "email" => &email,
                "regip" => &ip,
            })
            .ignore(&mut conn)
            .await?;

        crate::email::Email::send_registration(config, email, username, String::from_utf8_lossy(&hashed_password.to_vec()).to_string()).await?;

        Ok(warp::reply())
    }
    pub(crate) async fn activate_user(
        config: Arc<Config>,
        activation_key: String,
    ) -> Result<impl warp::Reply> {
        let mut conn = config.connect_db().await?;

        let decoded = String::from_utf8_lossy(
            &base64::engine::general_purpose::STANDARD.decode(activation_key)?,
        )
        .to_string();
        let key = decoded.split(':').collect::<Vec<_>>();

        let username = key[0];
        let hash = key[1];

        let test = "SELECT * FROM LEX_USERS WHERE UPPER(USRNAME) = :username AND USRPASS = :hash AND ISACTIVE = 'P'"
            .with(params! {
                username,
                hash,
            })
            .map(&mut conn, LEXUser::new)
            .await?;

        if test.len() == 1 {
            "UPDATE LEX_USERS SET ISACTIVE = 'T' WHERE UPPER(USRNAME) = :username AND USRPASS = :hash AND ISACTIVE = 'P'"
            .with(params! {
                username,
                hash,
            })
            .ignore(&mut conn)
            .await?;

            Ok(warp::reply())
        } else {
            Err(Error::Forbidden)
        }
    }
    pub(crate) async fn get_user(
        config: Arc<Config>,
        username: String,
        password: md5::Digest,
        ip: String,
        usrid: Option<usize>,
    ) -> Result<impl warp::Reply> {
        let mut conn = config.connect_db().await?;

        let id = if let Some(id) = usrid {
            id
        } else {
            Base::get_auth(config, username, password, ip).await?
        };

        let user = "SELECT * FROM LEX_USERS WHERE USRID = :usrid"
            .with(params! {
                "usrid" => id,
            })
            .map(&mut conn, LEXUser::new)
            .await?
            .first()
            .unwrap()
            .clone();

        let response = serde_json::json!({
            "id": user.usrid,
            "fullname": user.full_name,
            "username": user.username,
            // "registered": user.registered,
            // "last_login": user.last_login,
            "is_active": user.is_active,
            "user_level": user.usrlvl,
            "email": user.email_address,
            "login_count": user.login_count,
            "is_donator": user.donator,
            "is_rater": user.rater,
            "is_uploader": user.uploader,
            "is_author": user.author,
            "is_admin": user.is_admin,
        });

        Ok(warp::reply::json(&response))
    }

    pub(crate) async fn adm_get_user(
        config: Arc<Config>,
        username: String,
        password: md5::Digest,
        ip: String,
        usrid: usize,
    ) -> Result<impl warp::Reply> {
        let id = Base::get_auth(config.clone(), username.clone(), password, ip.clone()).await?;

        if Base::is_admin(config.clone(), id).await? {
            Ok(LEXUser::get_user(config, username, password, ip, Some(usrid)).await?)
        } else {
            Err(Error::Forbidden)
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn adm_get_all(
        config: Arc<Config>,
        username: String,
        password: md5::Digest,
        ip: String,
        rows_offset: u64,
        rows_count: u64,
        concise: bool,
    ) -> Result<impl warp::Reply> {
        let mut conn = config.connect_db().await?;
        let id = Base::get_auth(config.clone(), username.clone(), password, ip.clone()).await?;

        if Base::is_admin(config.clone(), id).await? {
            let users: Vec<serde_json::Value> = if concise {
                format!("SELECT USRID, USRNAME FROM LEX_USERS LIMIT {rows_offset}, {rows_count}")
                    .with(params! {
                        "usrid" => id,
                    })
                    .map(&mut conn, |(usrid, usrname): (String, String)| {
                        serde_json::json!({
                            "id" : usrid,
                            "username" : usrname,
                        })
                    })
                    .await?
            } else {
                format!("SELECT * FROM LEX_USERS LIMIT {rows_offset}, {rows_count}")
                    .with(params! {
                        "usrid" => id,
                    })
                    .map(&mut conn, LEXUser::new)
                    .await?
                    .into_iter()
                    .map(|usr| serde_json::to_value(usr).unwrap())
                    .collect()
            };

            Ok(warp::reply::json(&users))
        } else {
            Err(Error::Forbidden)
        }
    }

    pub(crate) async fn get_download_history(
        config: Arc<Config>,
        username: String,
        password: md5::Digest,
        ip: String,
    ) -> Result<impl warp::Reply> {
        let mut conn = config.connect_db().await?;
        let id = Base::get_auth(config.clone(), username.clone(), password, ip.clone()).await?;

        let history =
            "SELECT DT.LASTDL, DT.DLRECID, DT.USRID, DT.DLCOUNT, DT.LOTID, DT.VERSION, LL.LASTUPDATE
            FROM LEX_DOWNLOADTRACK DT INNER JOIN LEX_LOTS LL ON (DT.LOTID = LL.LOTID)
            WHERE DT.ISACTIVE = 'T' AND DT.USRID = :usrid AND DT.DLCOUNT >= 1
            ORDER BY LL.LASTUPDATE"
                .with(params! {
                    "usrid" => id,
                })
                .map(&mut conn, |
                        (lastdl, dlrecid, usrid, dlcount, lotid, version, lastupdate):
                            (String, String, String, String, String, String, String)
                    |
                    serde_json::json!({
                        "lastdl": lastdl,
                        "dlrecid": dlrecid,
                        "usrid": usrid,
                        "dlcount": dlcount,
                        "lotid": lotid,
                        "version": version,
                        "lastupdate": lastupdate,
                    })
                ).await?;

        let mut items = Vec::new();
        for item in history {
            let lot = "SELECT * FROM LEX_LOTS WHERE LOTID = :lotid"
                .with(params! {
                    "lotid" => item["lotid"].clone(),
                })
                .map(
                    &mut conn,
                    |(usrid, lotname, lastupdate, version, usrname): (
                        String,
                        String,
                        String,
                        String,
                        String,
                    )| {
                        serde_json::json!({
                            "usrid": usrid,
                            "lotname": lotname,
                            "lastupdate": lastupdate,
                            "version": version,
                            "usrname": usrname
                        })
                    },
                )
                .await?
                .first()
                .unwrap()
                .clone();

            let author = "SELECT USRNAME FROM LEX_USERS WHERE USRID = :usrid"
                .with(params! {
                    "usrid" => lot["usrid"].clone(),
                })
                .map(&mut conn, |usrname: String| {
                    serde_json::json!({
                        "username": usrname,
                    })
                })
                .await?
                .first()
                .unwrap()
                .clone();

            items.push(serde_json::json!({
                "lot": {
                    "id": lot["lotid"].clone(),
                    "name": lot["lotname"].clone(),
                    "update_date": lot["lastupdate"].clone(),
                    "version": lot["version"].clone(),
                    "author" : author["usrname"].clone(),
                },
                "record": {
                    "id": item["dlrecid"].clone(),
                    "last_downloaded": item["lastdl"].clone(),
                    "last_version": item["version"].clone(),
                    "download_count": item["dlcount"].clone(),
                },
            }));
        }

        Ok(warp::reply::json(&items))
    }

    pub(crate) async fn get_download_list(
        config: Arc<Config>,
        username: String,
        password: md5::Digest,
        ip: String,
    ) -> Result<impl warp::Reply> {
        let mut conn = config.connect_db().await?;
        let id = Base::get_auth(config.clone(), username.clone(), password, ip.clone()).await?;

        let history =
            "SELECT DT.LASTDL,DT.DLRECID,DT.USRID,DT.DLCOUNT,DT.LOTID,DT.VERSION,LL.LASTUPDATE
            FROM LEX_DOWNLOADTRACK DT INNER JOIN LEX_LOTS LL ON (DT.LOTID = LL.LOTID)
            WHERE DT.ISACTIVE = 'T' AND DT.USRID = :usrid AND DT.DLCOUNT = 0 AND LL.ISACTIVE = 'T' AND LL.ADMLOCK = 'F' AND LL.USRLOCK = 'F'
            ORDER BY LL.LASTUPDATE"
                .with(params! {
                    "usrid" => id,
                })
                .map(&mut conn, |
                        (lastdl, dlrecid, usrid, dlcount, lotid, version, lastupdate):
                            (String, String, String, String, String, String, String)
                    |
                    serde_json::json!({
                        "lastdl": lastdl,
                        "dlrecid": dlrecid,
                        "usrid": usrid,
                        "dlcount": dlcount,
                        "lotid": lotid,
                        "version": version,
                        "lastupdate": lastupdate,
                    })
                ).await?;

        let mut items = Vec::new();
        for item in history {
            let lot = "SELECT * FROM LEX_LOTS WHERE LOTID = :lotid"
                .with(params! {
                    "lotid" => item["lotid"].clone(),
                })
                .map(
                    &mut conn,
                    |(usrid, lotname, lastupdate, version, usrname): (
                        String,
                        String,
                        String,
                        String,
                        String,
                    )| {
                        serde_json::json!({
                            "usrid": usrid,
                            "lotname": lotname,
                            "lastupdate": lastupdate,
                            "version": version,
                            "usrname": usrname
                        })
                    },
                )
                .await?
                .first()
                .unwrap()
                .clone();

            let author = "SELECT USRNAME FROM LEX_USERS WHERE USRID = :usrid"
                .with(params! {
                    "usrid" => lot["usrid"].clone(),
                })
                .map(&mut conn, |usrname: String| {
                    serde_json::json!({
                        "username": usrname,
                    })
                })
                .await?
                .first()
                .unwrap()
                .clone();

            items.push(serde_json::json!({
                "lot": {
                    "id": lot["lotid"].clone(),
                    "name": lot["lotname"].clone(),
                    "update_date": lot["lastupdate"].clone(),
                    "version": lot["version"].clone(),
                    "author" : author["usrname"].clone(),
                },
                "record": {
                    "id": item["dlrecid"].clone(),
                    "last_downloaded": item["lastdl"].clone(),
                    "last_version": item["version"].clone(),
                    "download_count": item["dlcount"].clone(),
                },
            }));
        }

        Ok(warp::reply::json(&items))
    }
}
