from docker import DockerClient
from docker.models import containers
from ..utils import config
import pytest
from trueaccess.user import LinuxUser
from trueaccess.resources import LinuxHost

# from trueaccess.connections import LinuxConnect


class TestUser:
    def test_user_add(
        self,
        conftest_docker_client: DockerClient,
        conftest_config: config.Config,
        conftest_run_container: containers.Container,
        conftest_container_ip_address: str,
    ):
        linux_config = conftest_config.docker.linux
        master_user = LinuxUser(
            username=linux_config.master.username, password=linux_config.master.password
        )
        resource = LinuxHost(conftest_container_ip_address).authenticate_by_user_pwd(
            master_user
        )
        user = LinuxUser(
            username=linux_config.user.username, password=linux_config.user.password
        )
        user.add_to(resource)


class TestSomeOthre:
    def test_asdf(self):
        assert True
