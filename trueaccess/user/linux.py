from .base import User
from ..actions import AddUser
from ..resources import LinuxHost


class LinuxUser(User, AddUser):
    def add_to(self, resource: LinuxHost) -> bool:
        session = resource.session

        session.exec_command(f"useradd {self.username}")
