#!/usr/bin/env bash

pushd $(dirname $0) >/dev/null
cd ..

# Check if already installed...
if [ ! -d /opt/gcw0-toolchain ]
then
    echo "/opt/gcw0-toolchain not installed, downloading..."
    if [ ! -f opendingux-gcw0-toolchain.2014-08-20.tar.bz2 ]; then wget http://www.gcw-zero.com/files/opendingux-gcw0-toolchain.2014-08-20.tar.bz2; fi

    echo ""
    echo "######################################################################"
    echo "#                                                                    #"
    echo "#  Need to extract to /opt/gcw0-toolchain, sudo may need a password  #"
    echo "#                                                                    #"
    echo "######################################################################"
    echo ""
    sudo mkdir /opt 2>/dev/null

    echo "Extracting to /opt/gcw0-toolchain, this will take a few minutes..."
    sudo tar jxvf opendingux-gcw0-toolchain.2014-08-20.tar.bz2 -C /opt
    echo "All done!"

fi


if [ gcw0-stubs/libgcw0-stubs.c -nt gcw0-stubs/libgcw0-stubs.a ]; then
    echo "Building libgcw0-stubs.a..."
    pushd gcw0-stubs >/dev/null
    /opt/gcw0-toolchain/usr/bin/mipsel-linux-gcc -fPIC -O3 -c libgcw0-stubs.c -o libgcw0-stubs.o || exit 1
    /opt/gcw0-toolchain/usr/bin/mipsel-linux-ar rcv libgcw0-stubs.a libgcw0-stubs.o || exit 1
    #/opt/gcw0-toolchain/usr/bin/mipsel-linux-gcc -shared -fPIC -O3 libgcw0-stubs.c -o libgcw0-stubs.so || exit 1
    echo ""
    echo "######################################################################"
    echo "#                                                                    #"
    echo "#  Need to update to /opt/gcw0-toolchain, sudo may need a password   #"
    echo "#                                                                    #"
    echo "######################################################################"
    echo ""
    sudo cp libgcw0-stubs.a /opt/gcw0-toolchain/rust-gcw0-stubs.a
    popd >/dev/null
fi

popd >/dev/null
