use std::collections;

use crate::*;
use mysql_async::{
    params,
    prelude::{FromValue, Query, WithParams},
    Row,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Lot {
    lotid: isize,
    catid: isize,
    lotname: String,
    lotfile: String,
    usrid: isize,
    lotdesc: Vec<u8>,
    lotimgday: Vec<u8>,
    lotimgnigt: Vec<u8>,
    lotviews: isize,
    lotdownloads: isize,
    lastdownload: String,
    lastview: String,
    isactive: String,
    dateon: String,
    typeid: isize,
    dungeon: String,
    acclvl: isize,
    lastupdate: String,
    admlock: String,
    usrlock: String,
    lexexcl: String,
    searchlinks: Vec<u8>,
    rewardchain: Vec<u8>,
    maxiscat: String,
    biglotimg: Vec<u8>,
    version: String,
    lotgroup: isize,
    deps: String,
}

impl Lot {
    pub(crate) fn new(row: Row) -> Self {
        Lot {
            lotid: FromValue::from_value(row["lotid"].clone()),
            catid: FromValue::from_value(row["catid"].clone()),
            lotname: FromValue::from_value(row["lotname"].clone()),
            lotfile: FromValue::from_value(row["lotfile"].clone()),
            usrid: FromValue::from_value(row["usrid"].clone()),
            lotdesc: FromValue::from_value(row["lotdesc"].clone()),
            lotimgday: FromValue::from_value(row["lotimgday"].clone()),
            lotimgnigt: FromValue::from_value(row["lotimgnigt"].clone()),
            lotviews: FromValue::from_value(row["lotviews"].clone()),
            lotdownloads: FromValue::from_value(row["lotdownloads"].clone()),
            lastdownload: FromValue::from_value(row["lastdownload"].clone()),
            lastview: FromValue::from_value(row["lastview"].clone()),
            isactive: FromValue::from_value(row["isactive"].clone()),
            dateon: FromValue::from_value(row["dateon"].clone()),
            typeid: FromValue::from_value(row["typeid"].clone()),
            dungeon: FromValue::from_value(row["dungeon"].clone()),
            acclvl: FromValue::from_value(row["acclvl"].clone()),
            lastupdate: FromValue::from_value(row["lastupdate"].clone()),
            admlock: FromValue::from_value(row["admlock"].clone()),
            usrlock: FromValue::from_value(row["usrlock"].clone()),
            lexexcl: FromValue::from_value(row["lexexcl"].clone()),
            searchlinks: FromValue::from_value(row["searchlinks"].clone()),
            rewardchain: FromValue::from_value(row["rewardchain"].clone()),
            maxiscat: FromValue::from_value(row["maxiscat"].clone()),
            biglotimg: FromValue::from_value(row["biglotimg"].clone()),
            version: FromValue::from_value(row["version"].clone()),
            lotgroup: FromValue::from_value(row["lotgroup"].clone()),
            deps: FromValue::from_value(row["deps"].clone()),
        }
    }
    pub(crate) async fn get_all(config: Arc<Config>) -> Result<impl warp::Reply> {
        let mut conn = config.connect_db().await?;

        let lots: Vec<serde_json::Value> =
            "SELECT * FROM LEX_LOTS WHERE USRLOCK='F' AND ADMLOCK='F' AND ISACTIVE='T'"
                .with(())
                .map(&mut conn, |(lotid, lotname): (isize, String)| {
                    serde_json::json!({
                        "id": lotid,
                        "name": lotname,
                    })
                })
                .await?;

        Ok(warp::reply::json(&lots))
    }
    pub(crate) async fn get_lot(
        config: Arc<Config>,
        params: SearchParams,
        lot: Self,
        usrid: usize,
    ) -> Result<serde_json::Value> {
        let mut conn = config.connect_db().await?;
        let author = "SELECT * FROM LEX_USERS WHERE USRID = :usrid"
            .with(params! {"usrid" => usrid})
            .map(&mut conn, |usrname: String| usrname)
            .await?
            .first()
            .unwrap()
            .clone();

        let id = lot.lotid;
        let name = lot.lotname.trim();
        let version = lot.version.trim();
        let numdl = lot.lotdownloads;

        let exclusive = lot.lexexcl == "T";

        let desc = latin1_to_string(&lot.lotdesc);

        let img = serde_json::json!({
            "primary": format!("{}{}", config.img_link, String::from_utf8_lossy(&lot.lotimgday)),
            "secondary": format!("{}{}", config.img_link, String::from_utf8_lossy(&lot.lotimgnigt)),
            "extra": format!("{}{}", config.img_link, String::from_utf8_lossy(&lot.biglotimg)),
        });

        let link = format!("{}lex_filedesc.php?lotGET={}", config.index_link, id);
        let certified = lot.acclvl > 0;
        let active = !(lot.admlock == "T" || lot.usrlock == "T" || lot.isactive == "F");
        let upload_date = chrono::NaiveDateTime::parse_from_str(&lot.dateon, "%Y%m%d").unwrap();
        let update_date = chrono::NaiveDateTime::parse_from_str(&lot.lastupdate, "%Y%m%d").unwrap();
        let file = format!("{}{}", config.int_file_dir, lot.lotfile);
        let filesize = Lot::get_human_filesize(tokio::fs::read_to_string(file).await?).await;

        let comments = if params.comments == Some(true) {
            Some(Lot::get_comment(id).await?)
        } else {
            None
        };

        let votes = if params.votes == Some(true) {
            Some(Lot::get_vote(config.clone(), id).await?)
        } else {
            None
        };

        let dependencies = if params.dependencies == Some(true) {
            Some(Lot::get_dependencies(lot.deps.clone()).await?)
        } else {
            None
        };

        let categories = if params.categories == Some(true) {
            Some(Lot::get_categories(lot.clone()).await?)
        } else {
            None
        };

        let dependents = if params.dependents == Some(true) {
            Some(Lot::get_dependents(id).await?)
        } else {
            None
        };

        let user = if params.user == Some(true) {
            "'SELECT * FROM LEX_DOWNLOADTRACK WHERE LOTID = :lotid AND USRID = :usrid AND ISACTIVE=\'T\''"
                .with(params!{
                    "lotid" => id,
                    "usrid" => usrid
                }).map(&mut conn, |lastdl: String| lastdl).await?.first().cloned()
        } else {
            None
        };

        Ok(serde_json::json!({
            "id": id,
            "name": name,
            "version": version,
            "num_downloads": numdl,
            "author": author,
            "is_exclusive": exclusive,
            "description": desc,
            "images": img,
            "link": link,
            "is_certified": certified,
            "is_active": active,
            "upload_date": upload_date,
            "update_date": update_date,
            "filesize": filesize,
            "comments": comments,
            "votes": votes,
            "dependencies": dependencies,
            "categories": categories,
            "dependents": dependents,
            "last_downloaded": user,
        }))
    }
    pub(crate) async fn get_lot_http(
        config: Arc<Config>,
        lotid: isize,
    ) -> Result<impl warp::Reply> {
        let mut conn = config.connect_db().await?;

        let lot: Option<Lot> = "SELECT * FROM LEX_LOTS WHERE LOTID = :lotid"
            .with(params! {
                "lotid" => lotid,
            })
            .map(&mut conn, Lot::new)
            .await?
            .first()
            .cloned();

        match lot {
            Some(lot) => Ok(warp::reply::json(&lot)),
            None => Err(Error::NotFound),
        }
    }
    pub(crate) async fn check_download_limits(usr: String, lot: String) {
        todo!()
    }
    pub(crate) async fn update_download_tracker(usrid: String, lot: String) {
        todo!()
    }
    pub(crate) async fn get_download(lotid: isize) {
        todo!()
    }
    pub(crate) async fn do_download_list(lotid: isize) {
        todo!()
    }
    pub(crate) async fn delete_download_list(lotid: isize) {
        todo!()
    }
    pub(crate) async fn get_comment(lotid: isize) -> Result<serde_json::Value> {
        todo!()
    }
    pub(crate) async fn get_comment_http(lotid: isize) {
        todo!()
    }
    pub(crate) async fn get_vote(config: Arc<Config>, lotid: isize) -> Result<serde_json::Value> {
        let mut conn = config.connect_db().await?;

        let res =
            "SELECT * FROM LEX_VOTES WHERE LOTID = :lotid AND ISACTIVE = 'T' AND RATETYPE = 'U'"
                .with(params! {
                    "lotid" => lotid,
                })
                .map(&mut conn, |rating: isize| rating)
                .await?;

        let mut map = collections::HashMap::from([(1, 0), (2, 0), (3, 0)]);

        for rating in res {
            *map.get_mut(&rating).unwrap() += 1;
        }

        Ok(serde_json::json!({
            "1": map.get(&1).unwrap().clone(),
            "2": map.get(&2).unwrap().clone(),
            "3": map.get(&3).unwrap().clone(),
        }))
    }
    pub(crate) async fn get_vote_http(lotid: isize) {
        todo!()
    }
    pub(crate) async fn post_comment(lotid: isize) {
        todo!()
    }
    pub(crate) async fn get_categories(lot: Self) -> Result<serde_json::Value> {
        todo!()
    }
    pub(crate) async fn get_lot_dependency(lotid: isize) {
        todo!()
    }
    pub(crate) async fn get_dependency_string(lotid: isize) {
        todo!()
    }
    pub(crate) async fn update_dependency_string(lotid: isize) {
        todo!()
    }
    pub(crate) async fn bulk_download(lotid: isize) {
        todo!()
    }
    pub(crate) async fn get_dependencies_flat(deps: String) {
        todo!()
    }
    pub(crate) async fn get_dependencies(deps: String) -> Result<serde_json::Value> {
        todo!()
    }
    pub(crate) async fn get_dependents(lotid: isize) -> Result<serde_json::Value> {
        todo!()
    }
    pub(crate) async fn get_dependency_status(dep: String) {
        todo!()
    }
    pub(crate) async fn get_human_filesize(bytes: String) -> String {
        todo!()
    }
}
