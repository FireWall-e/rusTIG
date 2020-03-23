# rusTIG
**No dependency, cross-platform, native** Git CLI extension written in [Rust](https://www.rust-lang.org/).<br/>

***Works with users only!***
## How to use
**Download** main executable file from here: [rusTIG.exe](https://github.com/FireWall-e/rusTIG/raw/master/target/release/rusTIG.exe).<br/>
After download completed, **run it as administrator**.<br/>
<br/>In the terminal window you should be able to see following:
* path to your git executables folder;
* full paths to a newly created commands.

**After installation** you could simply **run new command as usual**, e.g. 'git crepo mynewrepo'
## List of commands
**Some commands may require additional auth data, such as GitHub username and [token](https://help.github.com/en/github/authenticating-to-github/creating-a-personal-access-token-for-the-command-line#creating-a-token) to access [GitHub API](https://developer.github.com/v3/).<br/>
It is recommended to run 'git rustig' once installation completed to setup environment.**
### git sur
**Sets up rusTIG's environment variables, such as GitHub username and [personal access token](https://help.github.com/en/github/authenticating-to-github/creating-a-personal-access-token-for-the-command-line#creating-a-token), within git config.**
### git crepo<br/>
**Creates GitHub repository**<br/>

Acceptable arguments:
- **repository name**<br/>
  e.g. 'git crepo mynewrepo' - creates new public repository 'mynewrepo'
  
- **--private** flag (true if provided)<br/>
  e.g. 'git crepo myprivaterepo --private' - creates new private repository 'mynewprivaterepo'
### git depo
**Deletes GitHub repository**<br/>

Acceptable arguments:
- **repository name**<br/>
  e.g. 'git depo myoldrepo' - deletes repository 'myoldrepo'
