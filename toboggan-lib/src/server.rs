use crate::{
    Database,
    Error,
};
use tarpc::{
    context,
};
use sled::Db;
use futures::{
    future::{self, Ready},
};
use std::{
    sync::Arc,
    ops::Deref,
};




#[derive(Clone,Debug)]
pub struct DbServer {
    pub db: Arc<Db>,
}
impl Database for DbServer {
    type TreeNamesFut = Ready<Vec<String>>;
    #[instrument(skip(self))]
    fn tree_names(self,_context:tarpc::context::Context) -> Self::TreeNamesFut {
        future::ready(self.db.tree_names().into_iter().map(|x| String::from_utf8(x.to_vec()).unwrap()).collect())
    }
    type NewTreeFut = Ready<Result<(), Error>>;
    #[instrument(skip(self))]
    fn new_tree(self,_context: context::Context, name:String) -> Self::NewTreeFut {
        match self.db.open_tree(&name) {
            Ok(_) => future::ready(Ok(())),
            Err(e) => future::ready(Err(e.into()))
        }
    }
    type GenerateIdFut = Ready<Result<u64, Error>>;
    #[instrument(skip(self))]
    fn generate_id(self,_context:tarpc::context::Context,) -> Self::GenerateIdFut {
        future::ready(self.db.generate_id().map_err(|e| e.into()))
    }
    type InsertFut = Ready<Result<Option<Vec<u8>>, Error>>;
    #[instrument(skip(self))]
    fn insert(self,_context:tarpc::context::Context,tree: Option<String>, key:Vec<u8>, value: Vec<u8>) -> Self::InsertFut {
        let tree_ref: &sled::Tree;
        let tree_owned: sled::Tree;
        match tree {
            Some(name) => {
                match self.db.open_tree(name) {
                    Ok(t) => { tree_owned = t; tree_ref = &tree_owned; },
                    Err(e) => return future::ready(Err(e.into())),
                }
            },
            _ => tree_ref = (*self.db).deref()
        }
        let res = tree_ref.insert(&key, &value[..]);
        info!("Inserted a value into key {:?}",key);
        
        match res {
            Ok(Some(prev)) => {
            future::ready(Ok(Some(prev.to_vec())))
            },
            Ok(None) => future::ready(Ok(None)),
            Err(e) => future::ready(Err(e.into()))
        }
    }
    type GetFut = Ready<Result<Option<Vec<u8>>, Error>>;
    #[instrument(skip(self))]
    fn get(self,_context:tarpc::context::Context,tree:Option<String> ,key:Vec<u8>) -> Self::GetFut {
        let tree_ref: &sled::Tree;
        let tree_owned: sled::Tree;
        match tree {
            Some(name) => {
                match self.db.open_tree(name) {
                    Ok(t) => { tree_owned = t; tree_ref = &tree_owned; },
                    Err(e) => return future::ready(Err(e.into())),
                }
            },
            _ => tree_ref = (*self.db).deref()
        }
        future::ready(
            match tree_ref.get(&key) {
                Ok(Some(v)) => Ok(Some(v.to_vec())),
                Ok(None) => Ok(None),
                Err(e) => Err(e.into())
            }
        )
    }
}
