import tomli
from ..config import Config


def config_loader(config_path):
    with open(config_path, "rb") as file:
        config_data = tomli.load(file)
    return Config(**config_data)
