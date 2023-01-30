from ..resources.base import Resource
from abc import ABC


class AddUser(ABC):
    def add_to(self, resource: Resource) -> bool:
        pass
