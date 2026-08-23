#!/usr/bin/env python3

import requests


class Account:
    """
    Account API used by the bank.
    """

    def __init__(self,
                 username: str,
                 password: str,
                 balance: float | None = 4000):
        self.__username = username
        self.__password = password
        self.__balance = balance

    def get_password(self) -> str:
        return self.__password

    def get_username(self) -> str:
        """
        Returns the username of my fake user.
        """
        return self.__username

    def get_balance(self) -> float:
        """
        Returns the balance of the user
        """
        return self.__balance

    def generate_json_request(self) -> str:
        """
        Creates a JSON representation of the Account
        """
        jsonreq: str = '{' +\
            f"\"username\":\"{self.__username}\",\n\"password\":\"{
                self.__password}\",\n\"balance\":{self.__balance} "+'}'
        return jsonreq


# change to ip of server when releasing
BANKURL = "http://127.0.0.1:8080/account/create"

if __name__ == "__main__":
    account = Account("totally_real_username", "totallyrealpassword", 3000)
    req = account.generate_json_request()
    bankaccount_create = requests.post(BANKURL, json=req)
    print(bankaccount_create)
