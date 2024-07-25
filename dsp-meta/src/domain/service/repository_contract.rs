use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[serde(rename = "_page")]
    pub page: usize,
    #[serde(rename = "_limit")]
    pub limit: usize,
}
impl Default for Pagination {
    fn default() -> Self {
        Pagination { page: 1, limit: 9 }
    }
}

/// The contract for the project metadata repository.
/// It defines the methods that the repository must implement.
/// The trait is generically typed for the entity type `Entity`, the id type `Id`, and
/// the error type `Error`.
pub trait RepositoryContract<Entity, Id, Error> {
    /// Retrieves an entity by its id.
    /// If the entity does not exist, `None` is returned.
    fn find_by_id(&self, id: &Id) -> Result<Option<Entity>, Error>;

    /// Returns all entities.
    fn find(&self, pagination: &Pagination) -> Result<Vec<Entity>, Error>;
}
