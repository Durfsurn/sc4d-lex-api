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
    deps: Vec<u8>,
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
    pub(crate) fn get_lot(config: Arc<Config>, lot: Self, usrid: usize) -> Self {
        // let mut conn = config.connect_db().await?;

        todo!()
    }
    pub(crate) async fn get_lot_http(
        config: Arc<Config>,
        lotid: String,
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
    pub(crate) async fn get_download(lotid: String) {
        todo!()
    }
    pub(crate) async fn do_download_list(lotid: String) {
        todo!()
    }
    pub(crate) async fn delete_download_list(lotid: String) {
        todo!()
    }
    pub(crate) async fn get_comment(lotid: String) {
        todo!()
    }
    pub(crate) async fn get_comment_http(lotid: String) {
        todo!()
    }
    pub(crate) async fn get_vote(config: Arc<Config>, lotid: String) -> Result<impl warp::Reply> {
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

        Ok(warp::reply::json(&serde_json::json!({
            "1": map.get(&1).unwrap().clone(),
            "2": map.get(&2).unwrap().clone(),
            "3": map.get(&3).unwrap().clone(),
        })))
    }
    pub(crate) async fn get_vote_http(lotid: String) {
        todo!()
    }
    pub(crate) async fn post_comment(lotid: String) {
        todo!()
    }
    pub(crate) async fn get_categories(lot: String) {
        todo!()
    }
    pub(crate) async fn get_lot_dependency(lotid: String) {
        todo!()
    }
    pub(crate) async fn get_dependency_string(lotid: String) {
        todo!()
    }
    pub(crate) async fn update_dependency_string(lotid: String) {
        todo!()
    }
    pub(crate) async fn bulk_download(lotid: String) {
        todo!()
    }
    pub(crate) async fn get_dependencies_flat(deps: String) {
        todo!()
    }
    pub(crate) async fn get_dependencies(deps: String) {
        todo!()
    }
    pub(crate) async fn get_dependents(lotid: String) {
        todo!()
    }
    pub(crate) async fn get_dependency_status(dep: String) {
        todo!()
    }
    pub(crate) async fn get_human_filesize(bytes: String, decimals: String) {
        todo!()
    }
}
