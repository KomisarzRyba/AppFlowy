use anyhow::Error;
use collab::core::collab::CollabDocState;

use flowy_document_deps::cloud::*;
use flowy_error::FlowyError;
use lib_infra::future::FutureResult;

pub(crate) struct LocalServerDocumentCloudServiceImpl();

impl DocumentCloudService for LocalServerDocumentCloudServiceImpl {
  fn get_document_doc_state(
    &self,
    _document_id: &str,
    _workspace_id: &str,
  ) -> FutureResult<CollabDocState, FlowyError> {
    FutureResult::new(async move { Ok(vec![]) })
  }

  fn get_document_snapshots(
    &self,
    _document_id: &str,
    _limit: usize,
    _workspace_id: &str,
  ) -> FutureResult<Vec<DocumentSnapshot>, Error> {
    FutureResult::new(async move { Ok(vec![]) })
  }

  fn get_document_data(
    &self,
    _document_id: &str,
    _workspace_id: &str,
  ) -> FutureResult<Option<DocumentData>, Error> {
    FutureResult::new(async move { Ok(None) })
  }
}
