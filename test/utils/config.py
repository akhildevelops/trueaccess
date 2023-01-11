from pydantic import BaseModel
from pathlib import Path


class LinuxDocker(BaseModel):
    dir_context: Path
    file_name: str
    image_name: str
    container_name: str


class Docker(BaseModel):
    linux: LinuxDocker


class Config(BaseModel):
    docker: Docker
