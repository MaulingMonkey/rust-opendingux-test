# Check if already installed...
if [ -d /opt/gcw0-toolchain ]; then exit 0; fi

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
