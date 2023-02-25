
pub fn clean_string(s: String) -> String {
    let email = s.split(":").collect::<Vec<&str>>()[1].to_string();
    email.replace('⟨', "").replace('⟩', "")
}