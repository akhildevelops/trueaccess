from ..utils import config
import pytest

from docker import DockerClient, errors as dockererrors
from docker.models.containers import Container


@pytest.fixture(autouse=True, scope="session")
def conftest_build_linux(
    conftest_docker_client: DockerClient, conftest_config: config.Config
) -> bool:
    linux_config = conftest_config.docker.linux
    yield conftest_docker_client.images.build(
        path=str(linux_config.dir_context),
        dockerfile=linux_config.file_name,
        tag=linux_config.image_name,
        rm=True,
    )
    # Every time the test tries to pull image from dokcerhub after deleting it.
    # Therefore disabling for a while.
    # conftest_docker_client.images.remove(image=linux_config.image_name)


@pytest.fixture(scope="module")
def conftest_run_container(
    conftest_docker_client: DockerClient, conftest_config: config.Config
) -> Container:
    linux = conftest_config.docker.linux
    try:
        container = conftest_docker_client.containers.get(linux.container_name)
    except dockererrors.NotFound:
        container = conftest_docker_client.containers.run(
            linux.image_name,
            name=linux.container_name,
            detach=True,
        )
    if container.status == "exited":
        container.start()
    yield container
    container.remove()


@pytest.fixture(scope="module")
def conftest_container_ip_address(
    conftest_docker_client: DockerClient,
    conftest_config: config.Config,
    conftest_run_container: Container,
) -> str:
    linux_config = conftest_config.docker.linux
    inspect_logs = conftest_docker_client.api.inspect_container(
        linux_config.container_name
    )
    ip_address = inspect_logs["NetworkSettings"]["IPAddress"]
    return ip_address
