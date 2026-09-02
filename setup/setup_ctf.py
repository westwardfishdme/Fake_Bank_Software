#!/usr/bin/env python3

import requests
import json
import random
import string

"""
Run this script when starting the ctf.

THIS SCRIPT SHOULD NOT BE INCLUDED INTO THE GIT REPOSITORY WHEN STARTING THE
ACTUAL CHALLENGES, AND MAY ONLY BE RUN ON THE MACHINE THAT WE ARE USING TO
SETUP THE CHALLENGES.
"""

IP= "192.168.0.68"
WEBSITE = f"http://{IP}:8080"
LOGIN = f"{WEBSITE}/login"
ACCOUNT = f"{WEBSITE}/account"
CREATE_ACCT = f"{ACCOUNT}/create"


# for type annotations.
class JSON:
    pass


# hashmap used for transactions later
# Stores the username:password
# as a key/value pair.
USERS = {}


class Requests:
    """
    Creates the JSON for requests
    sent to the server.
    """

    def new_password_request(
            username: str,
            old_password: str,
            new_password: str) -> JSON:
        """
        Changes the password for the username
        """
        USERS[username] = new_password
        json_req = '{'+f'"username":"{
            username}", "password":"{
            old_password}", "new_password":"{
            new_password}"'+'}'
        return json.loads(json_req)

    def create_acct_request(
            username: str,
            password: str,
            first_name: str,
            last_name: str,
            balance: int) -> JSON:
        """
        creates the json for an account, also stores the
        username/password inside of the USERS variable.
        """
        USERS[username] = password
        json_req = '{'+f'"username":"{
            username}", "password": "{
            password}", "first_name": "{
            first_name}", "last_name":"{
            last_name}", "init_balance":"{
            balance}"'+'}'
        return json.loads(json_req)


def setup_admin():
    """
    Sets up the admin account with the vulnerable password;
    changes the password to 'tokiohotel' (stored in rockyou.txt)
    as a reference to the library being used to create the async
    runtime.
    """
    print("Changing password to tokiohotel for admin...")
    json_req = Requests.new_password_request("admin", "password", "tokiohotel")
    # change the password
    requests.patch(f"{LOGIN}", json=json_req)


def generate_names() -> ([str], [str]):
    """
    generates the names from the provided text files.

    returns 2 lists in the following tuple:
    `(first names, last names)`
    """
    print("generating names...")
    first_names = []
    last_names = []
    with open("./us_first_names.txt", "r") as file:
        lines = file.readlines()
        for i in lines:
            # removes the newline character
            first_names.append(i[:-1])
    with open("./us_last_names.txt", "r") as file:
        lines = file.readlines()

        for i in lines:
            last_names.append(i[:-1])
    return (first_names, last_names)


def random_name(namelist: [str]) -> str:
    """
    returns a random name from a given list of names
    """
    last_index = namelist.__len__() - 1
    random_idx = random.randint(0, last_index)
    return namelist[random_idx]


def randompassword() -> str:
    """
    Generates a random password from a random string.
    """
    letters = string.ascii_letters
    password = ''.join(random.choice(letters) for i in range(12))
    return password


def create_username(first_name, last_name) -> str:
    """
    Creates a username with the following format:
    {first initial}{last name}{random 2 digit number}

    example:
    the username `jdoe74` would be generated for `John Doe`

    the username also gets pushed to the USERNAMES variable
    for later transaction use.
    """
    username_int = random.randint(0, 99)
    username_int = str(username_int) if username_int > 10 else f"0{
        username_int}"
    username = f"{first_name.lower()[0]}{last_name.lower()}{username_int}"
    return username


def setup_accounts(names_list: [str], lname_list: [str]):
    """
    Creates random users on the server. Will do so at random intervals
    of every 2 minutes to simulate a realistic server environment.
    """
    for i in range(100):
        first_name = random_name(names_list)
        last_name = random_name(lname_list)

        password = randompassword()
        balance = random.randint(300, 5000)
        username = create_username(first_name, last_name)

        json_req = Requests.create_acct_json(
            username, password, first_name, last_name, balance)
        print(json_req)
        # requests.post(f"{WEBSITE}/account/create", json=json_req)


def make_transcation(sender: str, recipient: str):
    """
    Makes a transaction on the server.
    """


class EasterEggs:
    """
    Creates some easter eggs in the transactions log, these include
    - A 'Mr. Robot' reference
    - 'iBelongHere67'
    """
    def mr_robot():
        """
        Creates the user 'samsepi0l' whose password is "fsociety"
        with the name "Elliot Alderson" who makes a transaction
        of 5272015 (premiere date of the first episode) to
        dalderson (darlene alderson)
        """

        first_episode = 5272015
        # create elliot's account
        elliot = Requests.create_acct_json("samsepi0l", "fsociety",
                                           "Elliot", "Alderson", first_episode)
        requests.post(f"{WEBSITE}/account/create", json=elliot)

        # create darlene's account
        last_episode = 12292019
        darlene = Requests.create_acct_json(
            "dalderson", "fsociety", "Darlene", "Alderson", last_episode)
        requests.post(f"{WEBSITE}/account/create", json=darlene)

        # make the transaction
        requests.post()

    def ibelonghere():
        """
        do nothing
        """


def exploit():
    """
    Runs Jordan's Exploit script (imported from a separate file.)
    """


if __name__ == "__main__":
    setup_admin()
    # (first_names_list, last_name_list) = generate_names()
    # setup_accounts(first_names_list, last_name_list)
