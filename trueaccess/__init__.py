from .default import CONFIG_PATH
from .utils.config import config_loader


config = config_loader(CONFIG_PATH)
