use bollard::{container, image, Docker};
use futures::future;
use futures_util::stream::StreamExt;
use rstest::*;
use tokio::runtime::Runtime;
use trueaccess::linux::user::Linux;
#[fixture]
fn docker_client() -> Docker {
    Docker::connect_with_local_defaults().unwrap()
}

const USERNAME: &str = "test";
const PASSWORD: &str = "test";
const PORT: u32 = 22;
const DOCKERIMAGE: &str = "talinux";
const DOCKERCONTAINER: &str = "talinux_cont";
const DOCKERFILEPATH: &str =
    "https://raw.githubusercontent.com/akhildevelops/AdventofCode2022/temp/Dockerfile";
macro_rules! rt_exec {
    ($fut:expr) => {{
        let rt = Runtime::new().unwrap();
        rt.block_on($fut)
    }};
}

#[fixture]
pub fn docker_linux(docker_client: Docker) -> Linux {
    let info = docker_client.inspect_container(DOCKERCONTAINER, None);
    let info = rt_exec!(info);
    if let Ok(list) = info {
        let settings = list.network_settings.unwrap().networks.unwrap();
        let hostname = &settings.get("bridge").unwrap().ip_address;
        if let Some(x) = hostname {
            Linux::new(USERNAME, PASSWORD, x, PORT)
        } else {
            panic!()
        }
    } else {
        if let Err(_) = rt_exec!(docker_client.start_container::<String>(DOCKERCONTAINER, None)) {
            let c_options = container::CreateContainerOptions {
                name: DOCKERCONTAINER,
            };
            let c_config = container::Config {
                image: Some(DOCKERIMAGE),
                ..Default::default()
            };
            if let Err(_) = rt_exec!(docker_client.create_container(Some(c_options), c_config)) {
                let img_options = image::BuildImageOptions {
                    remote: DOCKERFILEPATH,
                    t: DOCKERIMAGE,
                    ..Default::default()
                };
                let stream = docker_client.build_image(img_options, None, None);
                rt_exec!(async {
                    use std::env::current_dir;
                    println!("{:?}", current_dir());
                    stream
                        .for_each(|x| {
                            println!("{:?}", x);
                            future::ready(())
                        })
                        .await
                    // println!("{:?}", x);
                })
            }
        }
        docker_linux(docker_client)
    }
}
// impl Termination for Linux {
//     fn report(self) -> std::process::ExitCode {}
// }
