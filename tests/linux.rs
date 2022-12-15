use rstest::*;
mod fixtures;
use fixtures::linux::*;
use trueaccess::linux::user::Linux;
use trueaccess::user::User;
#[rstest]
fn check_connection(docker_linux: Linux) {
    docker_linux
        .create_session()
        .unwrap()
        .create_user("dumm23", "asdf")
        .unwrap();
}
