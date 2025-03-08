dkms=$DKMS_BIN_PATH

if [ ! -e "$dkms" ]; then
    echo "$dkms bin not found. Please see README"
    exit 1
fi
