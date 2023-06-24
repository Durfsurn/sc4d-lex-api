use crate::*;
use serde::{Deserialize, Serialize};

struct Search {}

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
               }
            } else {
                None
            }
        } else {
            None
        };

        let order_by = if let Some(ob) = params.order_by {
            if ob != "Update" {
                Some("LASTUPDATE >= 0".to_string())
            } else {
                None
            }
        } else {
            None
        };

        let group = if let Some(group) = params.group {
            if &group != "Select" && group.parse::<usize>().is_ok() {
                Some(format!(
                    "LOTGROUP = {}",
                    group.parse::<usize>().unwrap()
                ))
            } else {
                None
            }
        } else {
            None
        };

        let query = if let Some(query) = params.query {
            if &query != "Select" && !query.is_empty() {
                Some(format!(
                    "UPPER(LOTNAME) LIKE = {}",
                    query
                ))
            } else {
                None
            }
        } else {
            None
        };

        Ok(String::new())
    }
    pub(crate) fn do_search() {}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SearchParams {
    start: Option<isize>,
    amount: Option<isize>,
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
    exclude_notcert: Option<bool>,
    exclude_locked: Option<bool>,
}
