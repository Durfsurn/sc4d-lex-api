use crate::*;
use md5::Digest;
use mysql_async::prelude::{Query, WithParams};
use serde::{Deserialize, Serialize};

pub(crate) struct Search {}

impl Search {
    pub(crate) fn build_query(params: SearchParams) -> Result<String> {
        // bail if no filtering params present
        if params.creator.is_none()
            && params.broad_category.is_none()
            && params.lex_category.is_none()
            && params.lex_type.is_none()
            && params.broad_type.is_none()
            && params.group.is_none()
            && params.order_by.is_none()
            && params.query.is_none()
            && params.exclude_notcert.is_none()
            && params.exclude_locked.is_none()
        {
            return Err(Error::MalformedRequest);
        }

        let creator = if let Some(c) = params.creator {
            if &c != "Select" && c.parse::<usize>().is_ok() {
                Some(format!("USRID = {}", c.parse::<usize>().unwrap()))
            } else {
                None
            }
        } else {
            None
        };

        let broad_category = if let Some(bc) = params.broad_category {
            if &bc != "Select" {
                Some(format!("MAXISCAT = {}", bc))
            } else {
                None
            }
        } else {
            None
        };

        let lex_category = if let Some(lc) = params.lex_category {
            if &lc != "Select" && lc.parse::<usize>().is_ok() {
                Some(format!("CATID = {}", lc.parse::<usize>().unwrap()))
            } else {
                None
            }
        } else {
            None
        };

        let lex_type = if let Some(lt) = params.lex_type {
            if &lt != "Select" && lt.parse::<usize>().is_ok() {
                Some(format!("TYPEID = {}", lt.parse::<usize>().unwrap()))
            } else {
                None
            }
        } else {
            None
        };

        let broad_type = if let Some(bt) = params.broad_type {
            if &bt != "Select" {
                match bt.as_str() {
                    "lotbat" => Some("MAXISCAT IN ('250_MX_Agric.gif','250_MX_Civic.gif','250_MX_Comm.gif','250_MX_Ind.gif','250_MX_Lark.gif','250_MX_Parks.gif','250_MX_Res.gif','250_MX_Reward.gif','250_MX_Transport.gif','250_MX_Utility.gif','250_MXC_WFK-Canals.gif','250_MXC_Military.gif')"),
                    "dependency" => Some("MAXISCAT = '250_MXC_Dependency.gif'"),
                    "map" => Some("MAXISCAT = '250_MXC_Maps.gif'"),
                    "mod" => Some("MAXISCAT = '250_MXC_Modd.gif'"),
                    "other" => Some("MAXISCAT IN ('250_MXC_Tools.gif','250_MXC_FilesDocs.gif')"),
                    _ => None
               }.map(ToString::to_string)
            } else {
                None
            }
        } else {
            None
        };

        let last_updated = if let Some(ob) = &params.order_by {
            if ob != "Update" {
                Some("LASTUPDATE >= 0".to_string())
            } else {
                None
            }
        } else {
            None
        };

        let group = if let Some(g) = params.group {
            if &g != "Select" && g.parse::<usize>().is_ok() {
                Some(format!("LOTGROUP = {}", g.parse::<usize>().unwrap()))
            } else {
                None
            }
        } else {
            None
        };

        let query = if let Some(q) = params.query {
            if &q != "Select" && !q.is_empty() {
                Some(format!("UPPER(LOTNAME) LIKE = {}", q))
            } else {
                None
            }
        } else {
            None
        };

        let exclude_locked = if let Some(el) = params.exclude_locked {
            if el.parse::<bool>() == Ok(true) {
                Some("ADMLOCK = 'F' AND USRLOCK = 'F'".to_string())
            } else {
                None
            }
        } else {
            None
        };

        let exclude_notcert = if let Some(enc) = params.exclude_notcert {
            if enc.parse::<bool>() == Ok(true) {
                Some("ACCLVL > 0".to_string())
            } else {
                None
            }
        } else {
            None
        };

        let order_by = if let Some(ob) = params.order_by {
            match ob.as_str() {
                "download" => Some("ORDER BY LOTDOWNLOADS"),
                "popular" => Some("ORDER BY LOTDOWNLOADS"),
                "update" => Some("ORDER BY LASTUPDATE"),
                "recent" => Some("ORDER BY LOTID"),
                "random" => Some("ORDER BY RAND()"),
                _ => Some("ORDER BY LOTID"),
            }
        } else {
            Some("ORDER BY LOTID")
        }
        .map(ToString::to_string);

        let order = if let Some(o) = params.order {
            match o.to_uppercase().as_str() {
                "ASC" | "DESC" => Some(o),
                _ => Some("DESC".into()),
            }
        } else {
            None
        };

        let limit = {
            let start = params
                .start
                .and_then(|opt| opt.parse::<usize>().ok())
                .unwrap_or(0);
            let amount = params
                .amount
                .and_then(|opt| opt.parse::<usize>().ok())
                .unwrap_or(15);

            format!("LIMIT {start}, {amount}")
        };

        let where_clauses = &[
            creator,
            broad_category,
            lex_category,
            lex_type,
            broad_type,
            last_updated,
            group,
            query,
            exclude_locked,
            exclude_notcert,
            Some("ISACTIVE='T'".into()),
            order_by,
            order,
        ];

        let first_some = where_clauses
            .iter()
            .position(|wc| wc.is_some())
            .ok_or(Error::MalformedRequest)?;

        let clause = format!(
            "SELECT * FROM LEX_LOTS WHERE {} {} {limit}",
            where_clauses[first_some]
                .as_ref()
                .cloned()
                .unwrap_or_default(),
            where_clauses[first_some + 1..]
                .iter()
                .filter_map(|wc| wc.as_ref())
                .join(" AND ")
        );

        Ok(clause)
    }
    pub(crate) async fn do_search(
        config: Arc<Config>,
        username: String,
        password: Digest,
        ip: String,
        params: SearchParams,
    ) -> Result<impl warp::Reply> {
        let query = Search::build_query(params.clone())?;

        let mut conn = config.connect_db().await?;

        let user = Base::get_auth(config.clone(), username, password, ip).await?;

        let q = if params.concise {
            query
                .with(())
                .map(&mut conn, |(lotid, lotname): (String, String)| {
                    serde_json::json!({
                        "lotid": lotid,
                        "lotname": lotname,
                    })
                })
                .await?
        } else {
            query
                .with(())
                .map(&mut conn, crate::lot::Lot::new)
                .await?
                .into_iter()
                .map(|lot| {
                    let lot = crate::lot::Lot::get_lot(config.clone(), lot, user);

                    serde_json::to_value(lot).unwrap()
                })
                .collect()
        };

        Ok(warp::reply::json(&q))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct SearchParams {
    start: Option<String>,
    amount: Option<String>,
    order: Option<String>,
    concise: bool,
    user: Option<bool>,
    dependencies: Option<bool>,
    comments: Option<bool>,
    votes: Option<bool>,
    //  filtering params
    creator: Option<String>,
    broad_category: Option<String>,
    lex_category: Option<String>,
    lex_type: Option<String>,
    broad_type: Option<String>,
    group: Option<String>,
    order_by: Option<String>,
    query: Option<String>,
    exclude_notcert: Option<String>,
    exclude_locked: Option<String>,
}
