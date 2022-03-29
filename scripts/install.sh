echo "LibreHomework Installer"
echo "Getting latest version..."

LATREL=$(curl -L -s -H 'Accept: application/json' https://github.com/HGEpro/LibreHomework/releases/latest)
LATVER=$(echo $LATREL | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')


source="https://github.com/HGEpro/LibreHomework/archive/refs/tags/$LATVER.zip"
deb="https://github.com/HGEpro/LibreHomework/releases/download/$LATVER/librehomework_${LATVER}_amd64.deb"
appimage="https://github.com/HGEpro/LibreHomework/releases/download/$LATVER/librehomework_${LATVER}_amd64.AppImage"

bin_types=("Source" "Debian Package" "AppImage" "Skip to daemon installation")
select bin in "${bin_types[@]}"; do
    case $bin in
        "Source")
            echo "Downloading zip file..."
            curl -L -s $source -o "LibreHomework-Source_${LATVER}.zip"
            echo "Extracting zip file..."
            unzip "LibreHomework-Source_${LATVER}.zip" > /dev/null
            rm "LibreHomework-Source_${LATVER}.zip"
            echo "Done, you can now compile it by typing cd LibreHomework-Source_${LATVER} && npm i && npm run tauri build"
            break
            ;;
        "Debian Package")
            echo "Downloading debian package..."
            curl -L -s $deb -o "LibreHomework_${LATVER}.deb"
            echo "Enter the password to install the package:"
            sudo dpkg -i "LibreHomework_${LATVER}.deb" > /dev/null
            rm "LibreHomework_${LATVER}.deb"
            echo "Done, you can now launch it from the menu or the command line with libre-homework"
            break
            ;;
        "AppImage")
            echo "Downloading AppImage..."
            curl -L -s $appimage -o "LibreHomework_${LATVER}.AppImage"
            chmod +x "LibreHomework_${LATVER}.AppImage"
            echo "Done, you can now run it by typing ./LibreHomework_${LATVER}.AppImage"
	        break
            ;;
	"Skip to daemon installation")
	    echo "No binary has been downloaded"
	    break   
	    ;;
        *) echo "Invalid option $REPLY";;
    esac
done

echo "Do you want to download, install and configure the daemon? (y/n)"
read daemon
if [ $daemon = "y" ]; then

    echo "Downloading daemon executable..."
    curl -L -s "https://github.com/HGEpro/LibreHomework/releases/download/${LATVER}/librehomework-daemon_v1.0.0_amd64" -o "librehomework-daemon_${LATVER}_amd64"
    sudo mv "librehomework-daemon_${LATVER}_amd64" /usr/bin/librehomeworkd
    sudo chmod +x /usr/bin/librehomeworkd

    echo "The daemon will be run at boot with crontab. Checking installation..."


    if [ -f /etc/crontab ]; then
        echo "Crontab is installed"
        echo "Do you want to add the daemon to crontab? (y/n)"
        read crontab
    
        if [ $crontab = "y" ]; then

            if crontab -l | grep -q "/usr/bin/librehomeworkd"; then
                echo "The daemon is already in crontab"
            else
                echo "Adding the daemon to crontab..."
                crontab -l > crontab.tmp
                echo "@reboot /usr/bin/librehomeworkd" >> crontab.tmp
                crontab crontab.tmp
                rm crontab.tmp
                echo "Done, the daemon should start notifying the tasks soon. You can remove it by editing the crontab"
            fi
        else
            echo "The daemon won't be run at boot"
        fi
    else
        echo "Crontab is not installed, you can install it with sudo apt install cron"
    fi

else
    echo "No daemon has been downloaded"
fi