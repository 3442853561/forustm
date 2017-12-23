use sapper::{SapperModule, SapperRouter, Response, Request, Result as SapperResult, Error as SapperError };
use sapper_std::{render};
use super::super::{Redis, PubNotice, Permissions, WebContext};

pub struct WebAdminSection;

impl WebAdminSection {
    fn index(req: &mut Request) -> SapperResult<Response> {
        let web = req.ext().get::<WebContext>().unwrap().clone();
        res_html!("adminSection.html", web)
    }

    fn pub_notice(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<WebContext>().unwrap().clone();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        PubNotice::get(&mut web, &redis_pool);
        res_html!("adminPubNotice.html", web)
    }
}

impl SapperModule for WebAdminSection {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        let permission = req.ext().get::<Permissions>().unwrap();
        match permission {
            &Some(0) => Ok(()),
            _ => {
                let res = json!({
                    "status": false,
                    "error": String::from("Verification error")
                });
                Err(SapperError::CustomJson(res.to_string()))
            }
        }
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/admin/section", WebAdminSection::index);
        router.get("/admin/notice", WebAdminSection::pub_notice);

        Ok(())
    }
}
