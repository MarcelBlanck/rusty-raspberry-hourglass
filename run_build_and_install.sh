#!/bin/sh
cross build -j 6 --target arm-unknown-linux-gnueabihf \
&& ssh pi@hourglass "killall -9 hourglass" \
; ssh pi@hourglass "mkdir ~/html"
; scp target/arm-unknown-linux-gnueabihf/debug/hourglass pi@hourglass:~/ \
; scp html/index.html pi@hourglass:~/html \
; ssh pi@hourglass "~/hourglass"