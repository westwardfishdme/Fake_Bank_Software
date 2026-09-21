#!/usr/bin/env python3

import requests
import json
import random
import string
import time
import threading
import exploit
import sys
import os

"""
Run this script when starting the ctf.

THIS SCRIPT SHOULD NOT BE INCLUDED INTO THE GIT REPOSITORY WHEN STARTING THE
ACTUAL CHALLENGES, AND MAY ONLY BE RUN ON THE MACHINE THAT WE ARE USING TO
SETUP THE CHALLENGES.
"""

# testing server IP
IP = "localhost"
WEBSITE = f"http://{IP}:8080"
LOGIN = f"{WEBSITE}/login"
ACCOUNT = f"{WEBSITE}/account"
CREATE_ACCT = f"{ACCOUNT}/create"


# for type annotations.


def create_json(request: str) -> json:
    """
    Formats the string to proper json
    format.
    """
    return json.loads("{"+request+"}")


class Account:
    def __init__(self, username: str,
                 password: str, first_name: str | None = None,
                 last_name: str | None = None, balance: float | None = None):
        """
        Creates an account
        """
        self.__username = username
        self.__password = password
        self.__first_name = first_name
        self.__last_name = last_name
        self.__balance = balance

    def get_username(self) -> str:
        """
        Gets the account Username
        """
        return self.__username

    def get_balance(self) -> float:
        """
        Gets the current account balance.
        """
        return self.__balance

    def set_new_password(self, new_password: str):
        """
        Changes the password for the username
        """
        json_req = f'"username":"{
            self.__username}", "password":"{
            self.__password}", "new_password":"{
            new_password}"'
        response = requests.patch(
            LOGIN, json=create_json(json_req))
        print(response.content)

        # set the new password as the user's current password.
        self.__password = new_password

    def create(self):
        """
        creates the json for an account.
        """
        json_req = f'"username":"{
            self.__username}", "password": "{
            self.__password}", "first_name": "{
            self.__first_name}", "last_name":"{
            self.__last_name}", "init_balance":{
            self.__balance}'
        requests.post(CREATE_ACCT, json=create_json(json_req))

    def transaction(self, recipient: str, amount: float) -> json:
        """
        Makes a transaction on the server.
        """

        if amount > self.__balance:
            raise Exception("account would overdraw.")
        json_req = f'"username": "{self.__username}", "password": "{
            self.__password}", "recipient":"{recipient}", "amount": {amount}'
        requests.post(ACCOUNT, json=create_json(json_req))
        # update the balance.
        self.__balance -= amount

    def bad_account_information(self):
        """
        attempts a bad login between 1-2 times; then correctly logs in.
        """
        for i in range(random.randint(1, 2)):
            json_req = f'"username":"{self.__username}", "password": "{
                randompassword()}"'
            requests.get(ACCOUNT, json=create_json(json_req))
            time.sleep(2)
        self.account_information()

    def account_information(self):
        """
        Gets the account information of a user from the server.
        """
        json_req = f'"username":"{self.__username}", "password": "{
            self.__password}"'
        requests.get(ACCOUNT, json=create_json(json_req))


def generate_names() -> ([str], [str]):
    """
    generates the names from the provided text files.

    returns 2 lists in the following tuple:
    `(first names, last names)`
    """
    print("generating names...")
    first_names = []
    last_names = []
    try:
        with open("./us_first_names.txt", "r") as file:
            lines = file.readlines()
            for i in lines:
                # removes the newline character
                first_names.append(i[:-1])
        with open("./us_last_names.txt", "r") as file:
            lines = file.readlines()

            for i in lines:
                last_names.append(i[:-1])
    except FileNotFoundError:
        print(
            "[Error]: must run this script inside of the setup/ directory",
            file=sys.stderr)
        exit(1)
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


def create_username(first_name, last_name, add_numbers: bool = False) -> str:
    """
    Creates a username with the following format:
    {first initial}{last name}{random 2 digit number}

    example:
    the username `jdoe74` would be generated for `John Doe`

    the username also gets pushed to the USERNAMES variable
    for later transaction use.
    """
    username = f"{first_name.lower()[0]}{last_name.lower()}"
    if add_numbers:
        username_int = random.randint(0, 99)
        if add_numbers:
            username_int = str(username_int) if username_int > 10 else f"0{
                username_int}"
        username = f"{username}{username_int}"
    return username


# These are for doing random actions later.


class ImportantCTFaccounts:
    """
    Important CTF accounts that are used separately
    for narrative events. Used in other scripts to
    perform narrative actions.
    """

    def __init__(self):
        """
        Sets up the primary accounts for the CTF challenge.
        """
        self.__otherusers: dict[Account] = {}
        self.__usernames: list[str] = []

        # change admin pass
        self.__setup_admin()
        # setup other characters
        self.__setup_all()
        # setup jordan
        self.__setup_jordan()

    def __setup_all(self):
        """
        Creates Maya Caldwell's account
        """
        print("Setting up all the main characters in the story.")
        users = []

        # do this for every account.
        user: Account = Account("maya_caldwell", randompassword(),
                                "Maya", "Caldwell", 32_580)
        users.append(user)

        user: Account = Account("rholt", randompassword(),
                                "Richard", "Holt", 7_238_495)
        users.append(user)
        user: Account = Account("alex_turner10101011", randompassword(),
                                "Alex", "Turner", 23_540)
        users.append(user)
        user: Account = Account("priya_nath4994", randompassword(),
                                "Priya", "Nath", 53_540)
        users.append(user)
        user: Account = Account("sasha_oka4", randompassword(),
                                "Sasha", "Okafor", 32_440)
        users.append(user)

        for someuser in users:
            self.__usernames.append(someuser.get_username())
            self.__otherusers[someuser.get_username()] = someuser
            someuser.create()

    # TA
    def __setup_jordan(self):
        """
        Make Jordan's account
        This account is handled separately inside of his own
        exploit script + whatever we decide as a team should
        happen given his story.
        """

        first_name = "Jordan"
        last_name = "Mercer (flag: {ozX_B1Blica1})"
        username = f"{create_username(first_name, "Mercer")}"

        # found in wordlists/Passwords/bible.txt
        password = "commandments"
        jordan = Account(username, password,
                         first_name, last_name, 15_000)
        jordan.create()
        self.__usernames.append(username)
        self.__otherusers[username] = jordan

    def __setup_admin(self):
        """
        Sets up the admin account with the vulnerable password;
        changes the password to 'tokiohotel' (stored in rockyou.txt)
        as a reference to the library being used to create the async
        runtime.
        """
        print("Changing password to tokiohotel for admin...")
        # we have to set the new password manually because
        # in the actual code repository it is already created
        # with some default credentials as a red herring for
        # participants.
        admin: Account = Account("admin", "password")
        admin.set_new_password("tokiohotel")


class EasterEggs:
    """
    Creates some easter eggs in the transactions log, these include
    - A 'Mr. Robot' reference
    - 'iBelongHere67'
    """

    def __init__(self):
        """
        run the script to plant the easter eggs.
        """
        self.__mr_robot()

    def __mr_robot(self):
        """
        Creates the user 'samsepi0l' whose password is "fsociety"
        with the name "Elliot Alderson" who makes a transaction
        of 5272015 (premiere date of the first episode) to
        dalderson (darlene alderson)
        """

        first_episode = 5272015
        # create elliot's account
        elliot = Account("samsepi0l", "fsociety",
                         "Elliot", "Alderson", first_episode)
        elliot.create()

        # create darlene's account
        last_episode = 12292019
        darlene = Account(
            "dalderson", "fsociety", "Darlene", "Alderson", last_episode)
        darlene.create()

        # make the transaction
        elliot.transaction(darlene.get_username(), first_episode)


def jordan_exploit():
    """
    Runs Jordan's Exploit script (imported from a separate file.)
    """
    exploit.run()


def create_thread(func, *args, **kwargs) -> threading.Thread:
    return threading.Thread(target=func, args=(args), kwargs=kwargs)


def __random_user_second(first_idx: int, valid_users: list, user_dict: dict) -> Account:
    """
    In the case that we need to create a second user
    for an action in the `random_action()` function below,
    we need to create a second user.
    """
    valid_accounts = valid_users.__len__()-1
    idx_user2 = random.randint(0, valid_accounts)
    account2 = valid_users[idx_user2]

    if first_idx == idx_user2:
        # If the case is that the same user is selected;
        # we let __random_user_second() we use the adjacent person.
        match idx_user2:
            case 0:
                idx_user2 = 1
                account2 = valid_users[idx_user2]
                return user_dict.get(account2)
            case int(valid_accounts):
                idx_user2 = valid_accounts - 1
                account2 = valid_users[idx_user2]
                return user_dict.get(account2)
            case _:
                idx_user2 = first_idx+1
                account2 = valid_users[idx_user2]
                return user_dict.get(account2)
    return user_dict.get(account2)


def __random_action(action: str, valid_users: list, user_dict: dict):

    # get info for primary account
    idx_user1 = random.randint(0, valid_users.__len__()-1)
    user1 = valid_users[idx_user1]
    first_account: Account = user_dict.get(user1)

    # what does the user do?:
    match action:
        case "transfer":
            # in the case of transfers, take a random number
            # from 1-5% of the user's bank account and perform
            # a random transaction with another random user.
            second_account = __random_user_second(
                idx_user1, valid_users, user_dict)
            second_username = second_account.get_username()
            weight = random.randint(1, 5) / 100
            send_amt: float = first_account.get_balance() * weight
            print(f"\t{user1} is sending ${send_amt:.2f} to {second_username}")
            first_account.transaction(second_username, send_amt)
        case "check_balance":
            # just sends a get request to the server.
            print(f"\t{user1} is checking their balance...")
            first_account.account_information()
        case "change_password":
            # creates a new random password for the user.
            print(f"\t{user1} is changing their password...")
            first_account.set_new_password(randompassword())
        case "input_bad_password":
            # sometimes, we just mistype our password.
            print(f"\t{user1} is forgetting their password...")
            first_account.bad_account_information()


def user_do_random_action(valid_users: list, user_dict: dict):
    """
    have the accounts do random actions.
    """

    # flip a coin:
    # if 0 -> do nothing
    # if 1 -> do something,
    do_something = random.randint(0, 1)
    if do_something:
        actions = ["transfer", "check_balance",
                   "change_password", "input_bad_password"]
        act = random.randint(0, actions.__len__()-1)
        __random_action(actions[act], valid_users, user_dict)
    return


def setup_accounts(names_list: [str], lname_list: [str]):
    """
    Creates random users on the server. Will do so at random intervals
    of every 10 seconds to simulate a realistic server environment.

    Once at least 4 accounts are created; every 2 accounts generated
    will attempt to perform a random action.
    """

    random_actions: int = 200
    print("[setup] setting accounts up...")

    # local globals.
    user_dict: dict[Account] = {}
    valid_users: list[str] = []

    for i in range(100):
        first_name = random_name(names_list)
        last_name = random_name(lname_list)
        password = randompassword()
        balance = random.randint(300, 5000)
        username = f"{create_username(
            first_name, last_name, add_numbers=True)}"

        print(f"Creating user: {username} for {
              first_name} {last_name}; balance = {balance}")
        random_time = random.randint(0, 10)
        print(f"\ttime_to_create: {random_time} seconds")
        time.sleep(random_time)

        user = Account(username, password, first_name, last_name, balance)

        valid_users.append(username)  # add the username to a list
        # add the username and account information to a hashmap
        user_dict[username] = user
        user.create()

        if i >= 4 and i % 2 == 0:
            user_do_random_action(valid_users, user_dict)

    # once complete; try to see if someone wants to do something...
    for i in range(random_actions):
        user_do_random_action(valid_users, user_dict)
    print("primary script is done.")


if __name__ == "__main__":
    # test connection to server
    print("Checking connection to server...")
    site_status = requests.get(WEBSITE).status_code
    assert (site_status == 200)
    print(f"response ok: {site_status}")

    try:
        # setup primary ctf information
        primary = ImportantCTFaccounts()

        # random characters to generate names and stuff.
        (first_names_list, last_name_list) = generate_names()
        e = EasterEggs()

        threads = []

        # do this one separately. due to unique nature of having arguments.
        threads.append(threading.Thread(target=setup_accounts,
                       args=(first_names_list, last_name_list)))
        # run jordan's exploit simultaneously and create random transactions
        # simultaneously with the user_random function
        threads.append(create_thread(jordan_exploit))

        for t in threads:
            t.start()

        for t in threads:
            t.join()

    except KeyboardInterrupt:
        print("Ctrl-C pressed; deleting setup databases")
        rmauth = os.system("rm -r /tmp/auth_database/")
        rmuser = os.system("rm -r /tmp/user_database/")
        assert (rmauth == 0)
        assert (rmuser == 0)
        print("removed databases")

        os._exit(1)
