pub mod avatar;
pub mod display_name;
pub mod data_user_profile;

type Badges = i32;
type Flags = i32;

pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
    DisplayName
}