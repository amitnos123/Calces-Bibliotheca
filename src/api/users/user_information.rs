use reqwest;
use crate::data_model::{Badges, FieldsUser, Flags, user_status::UserStatus, avatar::Avatar, data_user_profile::DataUserProfile, display_name::DisplayName};

impl crate::Client {
    pub async fn fetch_self(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            super::create_url(),
            "@me"
        );
        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

    // target = user id
    pub async fn fetch_user(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            super::create_url(),
            target
        );
        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

    //TODO make tester
    pub async fn edit_user(
        &self,
        target: &str,
        avatar: Option<Avatar>,
        badges: Option<Badges>,
        display_name: Option<DisplayName>,
        flags: Option<Flags>,
        profile: Option<DataUserProfile>,
        remove: Vec<FieldsUser>,
        status: Option<UserStatus>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            super::create_url(),
            target
        );

        let body = EditUserRequestBody {
            avatar,
            badges,
            display_name,
            flags,
            profile,
            remove,
            status,
        };

        let rtn = self.reqwest_client.patch(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?;
        return Ok(rtn);
    }

    pub async fn fetch_user_flags(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}/{}",
            super::create_url(),
            target,
            "flags"
        );

        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

    pub async fn fetch_default_avatar(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}/{}",
            super::create_url(),
            target,
            "default_avatar"
        );

        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

    //TODO make tester
    pub async fn fetch_user_profile(&self, target: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}/{}",
            super::create_url(),
            target,
            "profile"
        );

        let rtn = self.reqwest_client.get(url)
            .header("x-bot-token", self.token.to_owned())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
        return Ok(rtn);
    }

}

// Helper structure to serialize the request body
#[derive(serde::Serialize)]
struct EditUserRequestBody {
    avatar: Option<Avatar>,
    badges: Option<Badges>,
    display_name: Option<DisplayName>,
    flags: Option<Flags>,
    profile: Option<DataUserProfile>,
    remove: Vec<FieldsUser>,
    status: Option<UserStatus>,
}