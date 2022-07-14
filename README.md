# cfuzzer

![](https://img.shields.io/badge/made%20with-Rust-orange)

### Description
This program will search for directories or subdomains by bruteforcing
given a URL and a file containing a list of directories. It will replace 
the keyword FUZZ with the respective entry in the file. And will omit requests
which lead to a 404 Not Found status code

---

### Table of Content
* Usage
* Contribute

---

### Usage
 What follows is an explanation of how to use this program

#### Compilation
The binary is already compiled and can be found in the current directory

---

#### Execution
This program takes 5 arguments:
```bash
./rfuzzer -u <url> -w <wordlist>
```
If by any reasons -u or -w weren't added or were bad written
an error message will be shown, this also includes forgetting the
keyword FUZZ, not adding http/https to the URL or trying to open an unexisting file.

---

#### Example
![cfuzzer-cap](https://user-images.githubusercontent.com/67020959/176977862-834dff9d-82f9-469c-8fca-42323306f323.png)

---

### Contribute
As always, I'm open to suggestions for trying to enhance the code and making
it more efficient :)
