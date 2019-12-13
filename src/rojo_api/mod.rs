//! Bindings to Rojo's API, exposed via Remodel.

mod project;

use rlua::{Context, UserData, UserDataMethods};

use crate::remodel_api::Remodel;

use project::RemodelProject;

pub struct RojoApi;

impl RojoApi {
    pub fn inject(context: Context<'_>) -> rlua::Result<()> {
        context.globals().set("rojo", Rojo)?;

        Ok(())
    }
}

struct Rojo;

impl UserData for Rojo {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("buildProject", |context, project: RemodelProject| {
            let tree = librojo::build_project(&project.0).unwrap();

            Remodel::import_tree_root(context, tree)
        });
    }
}
