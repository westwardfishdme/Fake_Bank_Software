# Unfinished Fake Bank Software
This directory holds the source code for the REST-API that Finch is building for
the WiCys x 7968 git challenge. Below is a check list of what needs to get done:

- Build and implement the REST-API server for the transactions.
- Build and implement a simple API for the web server to interact with.
- Set up the sample network.

Specific branches will NOT be publicly available and will be the code that is used on the transaction server.
The repository that will be accessible by competitors will have a similar code base. Branches that are production
ready will be disclosed at a later date.


## Where the vulnerabilities SHOULD lie:

- The REST-API doesn't have any token authentication and can be accessed publicly (user's will make a transfer from another user's account to retrieve the flag).
- The REST-API allows users to easily escalate privileges by allowing any user to forge a 
valid json request and give them privileges to run user-level commands (this is how they will retrieve another flag.)


