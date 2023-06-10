use std::collections::HashMap;
use std::ops::Deref;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use crate::data::{UserIdType, UserScoreType};

pub(crate) mod user;
lazy_static!{
    pub static ref USER_DATA: UserData = UserData::new();
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct UserDataPatch {
    id: UserIdType,
    hash: String,
    questions: HashMap<u8, QuestionsStatus>,
    score: UserScoreType
}

pub struct UserData {
    inner: RwLock<UserDataInner>
}

#[derive(Default, Clone, Debug, PartialEq)]
struct UserDataInner {
    id: UserIdType,
    hash: String,
    questions: HashMap<u8, QuestionsStatus>,
    score: UserScoreType
}
#[derive(Default, Clone, Debug, PartialEq)]
struct QuestionsStatus {
    question: Option<String>,
    is_correct: Option<bool>,
    right_answer: Option<String>,
    user_answer: Option<String>
}

impl UserData {
    fn new() -> Self {
        Self {
            inner: RwLock::new(UserDataInner::default())
        }
    }

    fn instance() -> &'static Self { &USER_DATA }

    async fn reset(&mut self) {
        let mut current_user_data = self.inner.write().await;
        current_user_data.id = UserIdType::default();
        current_user_data.hash = String::from("");
        current_user_data.questions = HashMap::new();
        current_user_data.score = UserScoreType::default();
    }

    async fn update_ser(new_user_data: UserDataPatch) -> std::io::Result<UserDataPatch> {

        let user_data = UserData::instance().inner.read().await.deref();

        if user_data == new_user_data {  }


        Ok(new_user_data)
    }
}