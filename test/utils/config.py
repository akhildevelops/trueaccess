from pydantic import BaseModel
from pathlib import Path


class User(BaseModel):
    username: str
    password: str


class LinuxDocker(BaseModel):
    dir_context: Path
    file_name: str
    image_name: str
    container_name: str
    user: User
    master: User


class Docker(BaseModel):
    linux: LinuxDocker


class Config(BaseModel):
    docker: Docker
