#!/usr/bin/env python3

import requests
import json


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
        self.__first_name = "John"
        self.__last_name = "Snow"
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

    def generate_json_request(self) -> requests:
        """
        Creates a JSON representation of the Account
        """
        jsonreq: str = '{' + f'"username":"{self.__username}","password":"{
            self.__password}","first_name":"{
            self.__first_name}","last_name":"{
            self.__last_name}","init_balance":{
            self.__balance}'+'}'
        jsonreq = json.loads(jsonreq)
        return jsonreq


# change to ip of server when releasing
BANKURL = "http://127.0.0.1:8080"

if __name__ == "__main__":
    account = Account("totally_real_username", "totallyrealpassword", 3000)
    req = account.generate_json_request()
    # try and see if we can contact the bank page
    try:
        response = requests.get(f"{BANKURL}")
    except Exception as e:
        print(f"failed to connect to bank server at {e}")
        exit(1)
    print("Connected to bank service...")

    response = requests.post(f"{BANKURL}/account/create", json=req)
    print(response.content)
