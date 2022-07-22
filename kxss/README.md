### KXSS
Get reflected XSS Parameters


```bash
$ echo http://target.com/?name=mike | kxss
param 'name' is reflected and allows ks>,ks<,ks" on http://target.com/
```
