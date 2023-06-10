pub trait ReplaceableUser {
    type UpdateData;

    fn update_user_data(&mut self, update_data: Self::UpdateData) -> bool;
}

//  TODO: finish and implement this trait into updating user data function
pub fn update_data_value<T: PartialEq>(current: &mut T, new: Option<T>, modified: &mut bool) {
    if let Some(new) = new {
        if *current != new {
            *modified = true;
        }
        *current = new;
    }
}