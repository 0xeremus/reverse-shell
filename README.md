# R&R Shell - Rusty Reverse Shell

I read [this blog post](https://ac1d.medium.com/how-to-build-a-go-reverse-shell-linux-windows-14288c358c9b) and realized this was pretty approachable to do in rust in very little time.

So here it is. Just a simple reverse shell listener that executes commands and returns output from either stdout or stderr.