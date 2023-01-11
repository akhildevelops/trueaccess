from docker import DockerClient
from ..utils import config
from typing import Any


class TestUser:
    def test_connection(
        self,
        conftest_docker_client: DockerClient,
        conftest_config: config.Config,
        conftest_run_container: Any,
    ):
        linux = conftest_config.docker.linux
        assert conftest_docker_client.containers.get(linux.container_name) is not None


class TestSomeOthre:
    def test_asdf(self):
        assert True
