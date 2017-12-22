use sapper::{SapperModule, SapperRouter, Response, Request, Result as SapperResult};
use sapper_std::{Context, render};
use super::super::{Permissions};
use util::{get_web_context};

pub struct Index;

impl Index {
    fn index(req: &mut Request) -> SapperResult<Response> {
        let web = get_web_context(req);
        res_html!("index.html", web)
    }

    fn login(req: &mut Request) -> SapperResult<Response> {
        let permission = req.ext().get::<Permissions>().unwrap().to_owned();
        let web = Context::new();
        match permission {
            Some(_) => {
                res_redirect!("/home")
            },
            None => {
                res_html!("login.html", web)
            }
        }
    }
}

impl SapperModule for Index {

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/", Index::index);

        router.get("/login", Index::login);

        Ok(())
    }
}
