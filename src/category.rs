use mysql_async::{
    prelude::{Query, WithParams},
};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Category {}

impl Category {
    pub(crate) async fn get_broad_category(config: Arc<Config>) -> Result<Vec<serde_json::Value>> {
        let mut conn = config.connect_db().await?;

        let cat = "SELECT * FROM LEX_MAXISTYPES WHERE ISACTIVE = 'T' ORDER BY MAXISCAT"
            .with(())
            .map(&mut conn, |(maxcnt, maxiscat, lotimg): (isize, String, String)| serde_json::json!({
                "id": maxcnt,
                "name": maxiscat,
                "image": lotimg,
            }))
            .await?;

        Ok(cat)
    }
    pub(crate) async fn get_lex_category(config: Arc<Config>) -> Result<Vec<serde_json::Value>> {
        let mut conn = config.connect_db().await?;

        let cat = "SELECT * FROM LEX_CATAGORIES WHERE ISACTIVE = 'T' ORDER BY CATNAME"
            .with(())
            .map(&mut conn, |(catid, catname): (isize, String)| serde_json::json!({
                "id": catid,
                "name": catname,
            }))
            .await?;

        Ok(cat)
    }
    pub(crate) async fn get_lex_type(config: Arc<Config>) -> Result<Vec<serde_json::Value>> {
        let mut conn = config.connect_db().await?;

        let cat = "SELECT * FROM LEX_TYPES WHERE ISACTIVE = 'T' ORDER BY TYPENAME"
            .with(())
            .map(&mut conn, |(typeid, typename, typedesc): (isize, String, String)| serde_json::json!({
                "id": typeid,
                "name": typename,
                "description": typedesc,
            }))
            .await?;

        Ok(cat)
    }
    pub(crate) async fn get_group(config: Arc<Config>) -> Result<Vec<serde_json::Value>> {
        let mut conn = config.connect_db().await?;

        let cat = "SELECT * FROM LEX_GROUPS INNER JOIN LEX_USERS ON LEX_GROUPS.AUTHOR = LEX_USERS.USRID WHERE LEX_GROUPS.ISACTIVE = 'T' ORDER BY NAME"
            .with(())
            .map(&mut conn, |(groupid, name, usrname): (isize, String, String)| serde_json::json!({
                "id": groupid,
                "name": name,
                "author": usrname,
            }))
            .await?;

        Ok(cat)
    }
    pub(crate) async fn get_author(config: Arc<Config>) -> Result<Vec<serde_json::Value>> {
        let mut conn = config.connect_db().await?;

        let cat = "SELECT USRID, USRNAME FROM LEX_USERS WHERE AUTHOR='T' ORDER BY USRNAME ASC"
            .with(())
            .map(&mut conn, |(usrid, usrname): (isize, String)| serde_json::json!({
                "id": usrid,
                "name": usrname,
            }))
            .await?;

        Ok(cat)

    }
    pub(crate) async fn get_all(config: Arc<Config>) -> Result<impl warp::Reply> {
        let get_broad_category = Category::get_broad_category(config.clone()).await?;
        let get_lex_category = Category::get_lex_category(config.clone()).await?;
        let get_lex_type = Category::get_lex_type(config.clone()).await?;
        let get_group = Category::get_group(config.clone()).await?;
        let get_author = Category::get_author(config.clone()).await?;

        Ok(warp::reply::json(&serde_json::json!({
            "broad_category": get_broad_category,
            "lex_category": get_lex_category,
            "lex_type": get_lex_type,
            "group": get_group,
            "author": get_author,
        })))
    }
}
