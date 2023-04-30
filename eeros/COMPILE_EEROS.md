[back](../)

**This should work, but for some reason it has thrown `GLIBCXX_...` and other errors. This may be due to a messed up local workspace.**

To have [EEROS](https://eeros.org) working on the BeagleBone Blue, you can either use an old image provided by OST or compile EEROS for the image provided on the BeagleBone blue already.
I am using QEMU for this, so you have to install the prerequisites depending on your distribution:

* `qemu-user` -  QEMU user mode emulation.
* `qemu-user-static` - Running QEMU not with an image but with all files in a folder.
* `qemu-user-static-binfmt` - Binary format rules for QEMU static user mode emulation - may conflict with the Non-Static binfmt package.

## Prepare the local environment

### 1. Prepare the BeagleBone

First you have to allow root access via SSH to clone the image:

```bash
$ sudo apt-get install git make cmake g++
$ sudo nano /etc/ssh/sshd_config
CTRL+W PermitRoot
PermitRootLogin yes #without-password
CTRL+X
y
ENTER
$ sudo systemctl restart sshd.service
$ exit
```

Update the system:

```bash
$ nano /etc/apt/sources.list
deb http://archive.debian.org/debian/ jessie main non-free
deb http://security.debian.org/ jessie/updates main
deb http://archive.debian.org/debian jessie-backports main
deb http://archive.debian.org/debian-archive/debian-security/ jessie updates/main non-free
```

Install CMake:

```bash
$ wget wget http://archive.debian.org/debian-archive/debian/pool/main/d/debian-archive-keyring/debian-archive-keyring_2017.5~deb8u1_all.deb
$ sudo dpkg -i debian-archive-keyring_2017.5~deb8u1_all.deb
$ sudo apt-get --allow-unauthenticated update
$ sudo apt-get --allow-unauthenticated upgrade
$ sudo apt-get install git make cmake g++
```

### 2. Synchronize the BeagleBone

Synchronize the whole image into a folder:

```bash
$ mkdir qemu_bbb
$ rsync -Wa --progress --exclude=/proc --exclude=/sys --exclude=/root --exclude=/tmp/ --exclude=/home/debian root@192.168.7.2:/* ./qemu_bbb/
$ cp $(which qemu-arm-static) ./qemu_bbb/usr/bin
```

### 3. Revoke root permisison

After the sync, revert the root permisison again:

```bash
$ sudo nano /etc/ssh/sshd_config
CTRL+W PermitRoot
PermitRootLogin without-password
CTRL+X
y
ENTER
$ sudo systemctl restart sshd.service
$ exit
```

## Work with the local environment

Now we have everything we need in the folder `qemu_bbb`.
So we can run the system with qemu inside there:

```bash
$ sudo chroot ./qemu_bbb /usr/bin/qemu-arm-static -cpu cortex-a8 /bin/bash
```
_(The parameter «-cpu cortex-a8» may not be needed)_


### Build EEROS from sources

On the host, not inside the QEMU-chroot:

```bash
$ cd qemu_bbb/opt/
$ mkdir eeros
$ cd eeros
$ git clone https://github.com/eeros-project/eeros-framework.git -b v1.4 eeros-framework
$ git clone https://github.com/eeros-project/bbblue-eeros.git -b v1.3 bbblue-eeros
$ mkdir -p eeros-framework/build-x86/eeros
$ mkdir -p bbblue-eeros/build-x86/eeros
```

After this, prepare it via cmake:

```bash
$ cd eeros-framework/build-x86/eeros/
$ cmake -DCMAKE_INSTALL_PREFIX=../../install-x86 ../..
```

#### Possible errors

* There may be some errors about cmake version 3.10. Just open each mentioned file, change the minimal version to 3.0 and run the command again.
* `Bad COMPATIBILITY value used for WRITE_BASIC_CONFIG_VERSION_FILE(): "SameMinorVersion"` - Change the value on Line 93 to `SameMajorVersion`
* Missing features from C++11: Add the value `-std=c++11` on line 90 to the compile option features parameter


