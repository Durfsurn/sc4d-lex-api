use mysql_async::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Lot {}

impl Lot {
    pub(crate) fn new(row: Row) -> Self {
        todo!()
    }
    pub(crate) async fn get_all() {
        todo!()
    }
    pub(crate) fn get_lot(lot: Self, usrid: usize) -> Self {
        todo!()
    }
    pub(crate) async fn get_lot_http(lotid: String) {
        todo!()
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
    pub(crate) async fn get_vote(lotid: String) {
        todo!()
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
