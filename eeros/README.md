
# EEROS Container-Build

I am using podman to have my containers and builds in usermode.

1. **Build** the image: `$ podman build --tag=eeros-build:1.0 .`
2. **Run** the Container with the project: `$ podman run --replace --name eeros-project-1 -v project/dir:/project:cached localhost/eeros-build:1.0`
3. Later just **Start** the Container to rebuild: `$ podman start -a eeros-project-1`
4. Check for the **binary**: `$ ls project/dir/build/`

## Example

There are two projects:

* `class` - Code used in the class for learning/demonstrating
* `playing` - Code used for playing around and try out other things

Each Project is C++-17 EEROS Project based on cmake.

To compile the playing project the first time:

```bash
# This has to be done only once.
# If the images is built and Dockerfile is not changed, omit
$ podman build --tag=eeros-build:1.0 .

# Staring the Container the first time
# Give it the name: eeros-playing
$ podman run --tty -i --replace --name eeros-class -v ./class:/project:cached localhost/eeros-build:1.0
root@...:/# ./build.sh

# After change some code and rebuild it by just starting the Container
$ podman start -a eeros-playing
root@...:/# ./build.sh
```

After this, copy over the binary and config file to the BeagleBone Blue:
```bash
$ scp -o HostKeyAlgorithms=+ssh-rsa build/eeros_test  ost@192.168.7.2:
$ scp -o HostKeyAlgorithms=+ssh-rsa build/HwConfigBBBlue.json  ost@192.168.7.2:
```
*(We need the old Host-Key algorithm due to the old image we use from OST...)*

Finally on the BeagleBone Blue, run it:
```bash
$ ssh -o HostKeyAlgorithms=+ssh-rsa ost@192.168.7.2
$ sudo ./eeros_test -c HwConfigBBBlue.json
```
