use sapper::{SapperModule, SapperRouter, Response, Request, Result as SapperResult};
use sapper_std::{Context, render, PathParams};
use super::super::{Postgresql};
use super::super::{Section, Article, RUser};
use uuid::Uuid;
use util::{get_web_context};

pub struct WebSection;

enum SectionTypes {
    Section = 0,
    Blog = 1,
}

impl WebSection {
    fn section(req: &mut Request) -> SapperResult<Response> {
        let mut web = get_web_context(req);
        let pg_conn = req.ext().get::<Postgresql>().unwrap().get().unwrap();

        let path_params = get_path_params!(req);

        let id: Uuid = match t_param!(path_params, "id").clone().parse() {
            Ok(i) => i,
            Err(err) => return res_400!(format!("UUID invalid: {}", err)),
        };
        let page = 1i64;
        web.add("id", &id);
        web.add("page", &page);

        let res = Section::query_with_section_id(&pg_conn, id);

        match res {
            Ok(r) => {
                if r.stype != SectionTypes::Section as i32 {
                    // return res_400!(format!("section not found {}, it's type is blog", r.id))
                    return res_redirect!(format!("/blog/{}", r.id));
                }

                web.add("res", &r);

                let articles = Article::query_articles_with_section_id_paging(&pg_conn, id, page, 20);
                match articles {
                    Ok(arts) => {
                        //println!("articles: {:?}", &arts);
                        web.add("articles", &arts.articles);
                        web.add("total", &arts.total);
                        web.add("max_page", &arts.max_page);

                        if let Some(suid) = r.suser {
                            let manager = RUser::query_with_id(&pg_conn, suid).unwrap();
                            web.add("manager", &manager);
                        } else {
                            web.add("manager", &false);
                        }

                        res_html!("detailSection.html", web)
                    }
                    Err(e) => res_500!(e),
                }
            }
            Err(e) => res_500!(format!("section not found: {}", e)),
        }
    }

    fn blog(req: &mut Request) -> SapperResult<Response> {
        let mut web = Context::new();
        let pg_conn = req.ext().get::<Postgresql>().unwrap().get().unwrap();

        let path_params = get_path_params!(req);

        let id: Uuid = match t_param!(path_params, "id").clone().parse() {
            Ok(i) => i,
            Err(err) => return res_400!(format!("UUID invalid: {}", err)),
        };
        let page = 1i64;
        web.add("id", &id);
        web.add("page", &page);

        let res = Section::query_with_user_id(&pg_conn, id);
        match res {
            Ok(r) => {
                if r.stype != SectionTypes::Blog as i32 {
                    // return res_400!(format!("section not found {}, it's type is section", r.id))
                    return res_redirect!(format!("/section/{}", r.id));
                }

                web.add("res", &r);

                let articles = Article::query_articles_with_section_id_paging(&pg_conn, id, page, 20);
                match articles {
                    Ok(arts) => {
                        //println!("articles: {:?}", &arts);
                        web.add("articles", &arts.articles);
                        web.add("total", &arts.total);
                        web.add("max_page", &arts.max_page);

                        if let Some(suid) = r.suser {
                            let manager = RUser::query_with_id(&pg_conn, suid).unwrap();
                            web.add("manager", &manager);
                        }

                        res_html!("detailSection.html", web)
                    }
                    Err(e) => res_500!(e),
                }
            }
            Err(e) => res_500!(format!("section not found: {}", e)),
        }
    }
}

impl SapperModule for WebSection {
    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/section/:id", WebSection::section);
        router.get("/blog/:id", WebSection::blog);

        Ok(())
    }
}
