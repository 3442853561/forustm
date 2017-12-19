use sapper::{SapperModule, SapperRouter, Response, Request, Result as SapperResult,
             Error as SapperError};
use sapper_std::{JsonParams, SessionVal};
use serde_json;

use super::super::{LoginUser, Permissions, Redis, ChangePassword, EditUser, Postgresql, RUser,
                   NewComment, DeleteComment, NewArticle, DeleteArticle};

pub struct User;

impl User {
    fn view_user(req: &mut Request) -> SapperResult<Response> {
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let mut res = json!({
                    "status": true,
                });
        res["data"] = serde_json::from_str(&RUser::view_with_cookie(redis_pool, cookie)).unwrap();
        res_json!(res)
    }

    fn edit(req: &mut Request) -> SapperResult<Response> {
        let body: EditUser = get_json_params!(req);
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let res = match body.edit_user(&pg_pool, redis_pool, cookie) {
            Ok(num_edit) => {
                json!({
                "status": true,
                "num_edit": num_edit
            })
            }
            Err(err) => {
                json!({
                "status": false,
                "error": err
            })
            }
        };
        res_json!(res)
    }

    fn change_pwd(req: &mut Request) -> SapperResult<Response> {
        let body: ChangePassword = get_json_params!(req);
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let res = match body.change_password(&pg_pool, redis_pool, cookie) {
            Ok(data) => {
                json!({
                    "status": true,
                    "data": data
                })
            }
            Err(err) => {
                json!({
                    "status": false,
                    "error": err
                })
            }
        };
        res_json!(res)
    }

    fn reset_pwd(req: &mut Request) -> SapperResult<Response> {
        #[derive(Deserialize, Serialize)]
        struct Account {
            account: String
        }
        let body: Account = get_json_params!(req);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let res = match RUser::reset_password(&pg_pool, body.account) {
            Ok(data) => {
                json!({
                    "status": true,
                    "data": data
                })
            }
            Err(err) => {
                json!({
                    "status": false,
                    "error": err
                })
            }
        };
        res_json!(res)
    }

    fn sign_out(req: &mut Request) -> SapperResult<Response> {
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let res = json!({"status": LoginUser::sign_out(redis_pool, cookie) });
        res_json!(res)
    }

    fn new_comment(req: &mut Request) -> SapperResult<Response> {
        let body: NewComment = get_json_params!(req);
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let res = match body.insert(&pg_pool, redis_pool, cookie) {
            true => {
                json!({
                "status": true
            })
            }
            false => {
                json!({
                "status": false
            })
            }
        };
        res_json!(res)
    }

    fn delete_comment(req: &mut Request) -> SapperResult<Response> {
        let body: DeleteComment = get_json_params!(req);
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let permission = req.ext().get::<Permissions>().unwrap();
        let res = match body.delete(&pg_pool, redis_pool, cookie, permission) {
            true => {
                json!({
                "status": true
            })
            }
            false => {
                json!({
                "status": false
            })
            }
        };
        res_json!(res)
    }

    fn new_article(req: &mut Request) -> SapperResult<Response> {
        let body: NewArticle = get_json_params!(req);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        if body.insert(&pg_pool, redis_pool, cookie) {
            res_json!(json!({"status": true}))
        } else {
            res_json!(json!({"status": false}))
        }
    }

    fn delete_article(req: &mut Request) -> SapperResult<Response> {
        let body: DeleteArticle = get_json_params!(req);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let permission = req.ext().get::<Permissions>().unwrap();
        let cookie = req.ext().get::<SessionVal>().unwrap();
        let res = match body.delete(&pg_pool, redis_pool, cookie, permission) {
            true => {
                json!({
                "status": true
            })
            }
            false => {
                json!({
                "status": false
            })
            }
        };
        res_json!(res)
    }
}

impl SapperModule for User {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        let permission = req.ext().get::<Permissions>().unwrap().to_owned();
        match permission {
            Some(_) => Ok(()),
            None => {
                let res = json!({
                    "status": false,
                    "error": String::from("Verification error")
                });
                Err(SapperError::CustomJson(res.to_string()))
            }
        }
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/user/sign_out", User::sign_out);

        router.get("/user/view", User::view_user);

        router.post("/user/reset_pwd", User::reset_pwd);

        router.post("/user/change_pwd", User::change_pwd);

        router.post("/user/edit", User::edit);

        router.post("/comment/new", User::new_comment);

        router.post("/comment/delete", User::delete_comment);

        router.post("/article/new", User::new_article);

        router.post("/article/delete", User::delete_article);

        Ok(())
    }
}
