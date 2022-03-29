echo "LibreHomework Uninstaller"
echo "This script will uninstall LibreHomework and remove the daemon from crontab"
echo "Do you want to continue? (y/n)"
read continue
if [ $continue = "y" ]; then

    if crontab -l | grep -q "/usr/bin/librehomeworkd"; then
        echo "The daemon is in crontab, removing it..."
        crontab -l > crontab.tmp

        line=$(grep -n 'librehomeworkd' crontab.tmp)
        sed ${line:0:1}d cron.tmp > _crontab.tmp

        crontab -r
        crontab _crontab.tmp
        rm crontab.tmp
        echo "Done"
    else
        echo "The daemon is not in crontab"
    fi

    if [ -f /usr/bin/librehomeworkd ]; then
        echo "Removing the daemon from /usr/bin/librehomeworkd"
        sudo rm /usr/bin/librehomeworkd
        echo "Done"
    else
        echo "The daemon is not installed"
    fi

    if [ -z "$XDG_CONFIG_HOME" ]; then
        XDG_CONFIG_HOME="$HOME/.config"
        echo "Done"
    fi

    if [ -d "$XDG_CONFIG_HOME/LibreHomework" ]; then
        echo "Removing the configuration directory"
        rm -rf "$XDG_CONFIG_HOME/LibreHomework"
        echo "Done"
        
    else
        echo "The configuration files are not installed"
    fi

    #check if /usr/bin/libre-homework exists
    if [ -f /usr/bin/libre-homework ]; then
        echo "Removing the app /usr/bin/libre-homework with apt, please enter the password:"
        sudo apt remove libre-homework -y
        echo "Done"
    else
        echo "The app is not installed"
    fi

    echo "LibreHomework has been completely uninstalled"
else
    echo "Aborting"
fi