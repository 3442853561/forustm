pub mod articles;
pub mod comments;
pub mod sections;
pub mod rusers;

pub(crate) use self::articles::{Article, NewArticle, EditArticle, DeleteArticle};
pub(crate) use self::comments::{Comment, NewComment, DeleteComment};
pub(crate) use self::rusers::{RUser, LoginUser, EditUser, ChangePassword, ChangePermission,
                              RegisteredUser};
pub(crate) use self::sections::{InsertSection, Section};

use uuid::Uuid;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangStatus {
    pub id: Uuid,
    pub status: i16,
}
