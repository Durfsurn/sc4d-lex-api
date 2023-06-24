struct Lot {}

impl Lot {
    pub(crate) fn getAll() {}
    pub(crate) fn getLot(lot: String, user: String) {}
    pub(crate) fn getLotHttp(lotid: String) {}
    pub(crate) fn checkDownloadLimits(usr: String, lot: String) {}
    pub(crate) fn updateDownloadTracker(usrid: String, lot: String) {}
    pub(crate) fn getDownload(lotid: String) {}
    pub(crate) fn doDownloadList(lotid: String) {}
    pub(crate) fn deleteDownloadList(lotid: String) {}
    pub(crate) fn getComment(lotid: String) {}
    pub(crate) fn getCommentHttp(lotid: String) {}
    pub(crate) fn getVote(lotid: String) {}
    pub(crate) fn getVoteHttp(lotid: String) {}
    pub(crate) fn postComment(lotid: String) {}
    pub(crate) fn getCategories(lot: String) {}
    pub(crate) fn getLotDependency(lotid: String) {}
    pub(crate) fn getDependencyString(lotid: String) {}
    pub(crate) fn updateDependencyString(lotid: String) {}
    pub(crate) fn bulkDownload(lotid: String) {}
    pub(crate) fn getDependenciesFlat(deps: String) {}
    pub(crate) fn getDependencies(deps: String) {}
    pub(crate) fn getDependents(lotid: String) {}
    pub(crate) fn getDependencyStatus(dep: String) {}
    pub(crate) fn getHumanFilesize(bytes: String, decimals: String) {}
}
