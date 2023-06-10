pub(crate) mod user;

use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::datatypes::{ResponseCodes, UserIdType, UserScoreType};
use crate::traits;
use crate::traits::ReplaceableUser;

lazy_static!{
    pub static ref USER_DATA: UserData = UserData::new();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////   STRUCTS   ////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct UserData {
    pub inner: RwLock<UserDataInner>
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct UserDataPatch {
    id: UserIdType,
    hash: String,
    questions: HashMap<u8, QuestionsStatus>,
    score: UserScoreType
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDataInner {
    id: UserIdType,
    hash: String,
    questions: HashMap<u8, QuestionsStatus>,
    score: UserScoreType
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionsStatus {
    question: Option<String>,
    is_correct: Option<bool>,
    right_answer: Option<String>,
    user_answer: Option<String>
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////   IMPLs   /////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

impl UserData {
    fn new() -> Self {
        Self {
            inner: RwLock::new(UserDataInner::default())
        }
    }

    pub fn instance() -> &'static Self { &USER_DATA }

    pub fn get_inner() -> &'static RwLock<UserDataInner> {
        &Self::instance().inner
    }

    async fn reset_user_data(&mut self) {
        let mut current_user_data = self.inner.write().await;
        current_user_data.id = UserIdType::default();
        current_user_data.hash = String::from("");
        current_user_data.questions = HashMap::new();
        current_user_data.score = UserScoreType::default();
    }

    pub async fn update_user(new_user_data: UserDataPatch) -> std::io::Result<ResponseCodes> {

        let mut user_data = Self::instance().inner.write().await;

        let modified = user_data.update_user_data(new_user_data);

        if modified {
            Ok(ResponseCodes::Ok)
        } else {
            Ok(ResponseCodes::NotModified)
        }
    }
}

impl UserDataPatch {
    pub fn new(
        id: UserIdType,
        hash: String,
        questions: HashMap<u8, QuestionsStatus>,
        score: UserScoreType
    ) -> Self {
        Self {
            id,
            hash,
            questions,
            score
        }
    }
}

impl ReplaceableUser for UserDataInner {
    type UpdateData = UserDataPatch;

    fn update_user_data(&mut self, update_data: Self::UpdateData) -> bool {
        let mut modified = false;
        traits::update_data_value(&mut self.id, Some(update_data.id), &mut modified);
        traits::update_data_value(&mut self.hash, Some(update_data.hash), &mut modified);
        traits::update_data_value(&mut self.questions, Some(update_data.questions), &mut modified);
        traits::update_data_value(&mut self.score, Some(update_data.score), &mut modified);

        modified
    }
}

impl QuestionsStatus {
    pub fn new(
        question: Option<String>,
        is_correct: Option<bool>,
        right_answer: Option<String>,
        user_answer: Option<String>
    ) -> Self {
        Self {
            question,
            is_correct,
            right_answer,
            user_answer
        }
    }
}