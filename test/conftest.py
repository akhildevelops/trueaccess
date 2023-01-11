import docker
import tomli
import pytest
from .utils import config
from pathlib import Path
from docker import DockerClient

CONFIG_FILE = Path("test", "config.toml")


@pytest.fixture(scope="session")
def conftest_docker_client() -> DockerClient:
    return docker.from_env()


@pytest.fixture(scope="session")
def conftest_config() -> config.Config:
    with open(CONFIG_FILE, "rb") as file:
        test_config = tomli.load(file)
    return config.Config(**test_config)
