from .. import config
from .base import Resource
from ..actions.authenticate import AuthenticatebyUserPwd
from typing import Optional
from paramiko.client import SSHClient
from ..user.base import User


class LinuxHost(Resource, AuthenticatebyUserPwd):
    def __init__(self, hostname: str, ssh_port=config.resource.linux.ssh_port) -> None:
        self.hostname = hostname
        self.ssh_port = ssh_port
        self.session: Optional[SSHClient] = None

    def authenticate_by_user_pwd(self, user: User):
        super().authenticate_by_user_pwd(user)
        self.session = SSHClient()
        self.session.connect(
            self.hostname, self.ssh_port, username=user.username, password=user.password
        )
        return self
