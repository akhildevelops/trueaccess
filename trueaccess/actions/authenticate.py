from ..user.base import User

from abc import ABC


class BaseAuthenticate:
    pass


class AuthenticatebyUserPwd(ABC, BaseAuthenticate):
    def authenticate_by_user_pwd(self, user: User):
        pass
