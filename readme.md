# Unfinished Fake Bank Software
This directory holds the source code for the REST-API that Finch is building for
the WiCys x 7968 git challenge. Below is a check list of what needs to get done:

- Build and implement a simple API for the web server to interact with.
- Set up the sample network.

Specific branches will NOT be publicly available and will be the code that is used on the transaction server.
The repository that will be accessible by competitors will have a similar code base. Branches that are production
ready will be disclosed at a later date.

These branches are:
  - main

The branch we will upload publicly is:
  - release (not yet created)

# Flags:

### Implemented
- Anyone can pull any User's authentication information; including the password hash + the salt value used to calculate the hash. 
This will be a flag for the admin account! (implemented) 

### Partially Implemented
- Users will be able to create accounts with any set balance. (partially implemented)

### Not Yet Implemented
- Users will be able to log into one of jordan's actual account and be able to trace the account history to find 
dummy accounts to find money trail. (flag not implemented)
- Users will be able to see commit history and get a flag there (flag not implemented)
- Users will be able to switch to an experimental branch with the flag in the readme (flag not implemented)

## Some Easter eggs
;) python script has more information

# Readme
the actual readme that will be uploaded to the github page is located inside of this directory here:
- [ readme ](./docs/README.md)
