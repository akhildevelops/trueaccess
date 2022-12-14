use crate::user::TAUser;
use regex::Regex;

const RE_USER_ALREADY_EXISTS: &str = r"useradd: user '(.+)' already exists";
pub const USER_ADD: &str = "useradd";
pub const ETC_PASSWD: &str = "/etc/passwd";

pub fn user_exist_from_response(s: &str, user: Option<&TAUser>) -> bool {
    let re = Regex::new(RE_USER_ALREADY_EXISTS).unwrap();
    match user {
        Some(x) => re.captures(s).map_or(false, |y| {
            y.get(1).map_or(false, |z| z.as_str() == x.username)
        }),
        None => re.is_match(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_exist() {
        let user = TAUser::new("true", "access");
        assert_eq!(
            user_exist_from_response("useradd: user 'true' already exists", Some(&user)),
            true,
        );
    }

    #[test]
    fn test_user_none_exist() {
        assert_eq!(
            user_exist_from_response("useradd: user 'true' already exists", None),
            true,
        );
    }

    #[test]
    fn test_user_none_exist_none() {
        assert_eq!(user_exist_from_response("user exists", None), false);
    }

    #[test]
    fn test_user_exist_none() {
        let user = TAUser::new("true", "access");
        assert_eq!(user_exist_from_response("user exists", Some(&user)), false);
    }
}
