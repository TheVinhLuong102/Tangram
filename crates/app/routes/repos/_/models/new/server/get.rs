use crate::page::Page;
use anyhow::{bail, Result};
use pinwheel::prelude::*;
use std::sync::Arc;
use tangram_app_common::{
	error::{not_found, redirect_to_login, service_unavailable},
	path_components,
	user::{authorize_user, authorize_user_for_repo},
	Context,
};
use tangram_app_layouts::app_layout::app_layout_info;
use tangram_id::Id;

pub async fn get(request: &mut http::Request<hyper::Body>) -> Result<http::Response<hyper::Body>> {
	let context = Arc::clone(request.extensions().get::<Arc<Context>>().unwrap());
	let repo_id = if let ["repos", repo_id, "models", "new"] = *path_components(request).as_slice()
	{
		repo_id.to_owned()
	} else {
		bail!("unexpected path");
	};
	let mut db = match context.database_pool.begin().await {
		Ok(db) => db,
		Err(_) => return Ok(service_unavailable()),
	};
	let user = match authorize_user(request, &mut db, context.options.auth_enabled()).await? {
		Ok(user) => user,
		Err(_) => return Ok(redirect_to_login()),
	};
	let repo_id: Id = match repo_id.parse() {
		Ok(repo_id) => repo_id,
		Err(_) => return Ok(not_found()),
	};
	if !authorize_user_for_repo(&mut db, &user, repo_id).await? {
		return Ok(not_found());
	}
	let app_layout_info = app_layout_info(&context).await?;
	let page = Page {
		app_layout_info,
		error: None,
	};
	let html = html(page);
	let response = http::Response::builder()
		.status(http::StatusCode::OK)
		.body(hyper::Body::from(html))
		.unwrap();
	Ok(response)
}
