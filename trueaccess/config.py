from pydantic import BaseModel


class LinuxResource(BaseModel):
    ssh_port: int


class Resource(BaseModel):
    linux: LinuxResource


class Config(BaseModel):
    resource: Resource
