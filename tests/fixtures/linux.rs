use std::{collections::HashMap, thread, time};

use bollard::{container, image, models, Docker};
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
const DOCKER22PORTBINDING: u32 = 4567;
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
        if list.state.unwrap().running.unwrap() {
            Linux::new(USERNAME, PASSWORD, "localhost", DOCKER22PORTBINDING)
        } else {
            rt_exec!(docker_client.restart_container(DOCKERCONTAINER, None)).unwrap();
            docker_linux(docker_client)
        }
    } else {
        if let Err(_) = rt_exec!(docker_client.start_container::<String>(DOCKERCONTAINER, None)) {
            let p_b = models::PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some(DOCKER22PORTBINDING.to_string()),
            };
            let mut p_m = HashMap::new();
            p_m.insert("22/tcp".to_string(), Some(vec![p_b]));
            let c_options = container::CreateContainerOptions {
                name: DOCKERCONTAINER,
            };
            let host_config = models::HostConfig {
                port_bindings: Some(p_m),
                ..Default::default()
            };
            let c_config = container::Config {
                image: Some(DOCKERIMAGE),
                host_config: Some(host_config),
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
                })
            }
        }
        docker_linux(docker_client)
    }
}
