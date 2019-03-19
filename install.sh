#!/usr/bin/env bash
set -e

architecture() {
    case `uname -m` in
        x86_64)
            echo x86_64
            ;;
        i686 | i386)
            echo i686
            ;;
        #Probably expand out more options for arm and aarch64
        armv7*)
            echo arm32
            ;;
        aarch64)
            echo arm64
            ;;
        *)
            error "unknown architecture detected"
            ;;
    esac
}

get_os() {
    case `uname -s` in
        Linux)
            case `uname -o` in
                Android)
                    echo Android
                    ;;
                GNU/Linux)
                    echo Linux
                    ;;
            esac
            ;;
        MINGW* | MSYS* | CYGWIN*)
            echo Windows
            ;;
        Darwin)
            echo Darwin
            ;;
    esac
}

system() {
    case $(get_os) in
        Linux)
            echo linux
            ;;
        Android)
            echo android
            ;;
        Darwin)
            echo apple
            ;;
        Windows)
            echo pc-windows-msvc
            ;;
        *)
            error "machine os type is not supported"
            ;;
    esac
}

get_latest_version() {
  curl --silent "https://api.github.com/repos/foundpatterns/torchbear/releases/latest" |
    grep '"tag_name":' |
    sed -E 's/.*"([^"]+)".*/\1/'
}

get_url() {
    local arch=$(architecture)
    local os=$(system)
    #Maybe instead of getting the latest version, we could get the latest stable release instead to reduce the chance of
    #exposed bugs being sent to users
    local version=$(get_latest_version)
    #TODO: Use github api to get the uri for the download instead.
    echo "https://github.com/foundpatterns/torchbear/releases/download/${version}/torchbear-${version}-${arch}-${os}-stable.zip"
}

download_and_extract() {
    if [ ! -d $1 ]; then
        error "Path or directory does not exist."
    fi

    if [ -x "$(command -v curl)" ]; then
        curl -L $(get_url) -o temp.zip
        unzip -o temp.zip -d $1
        rm temp.zip
    else
        error "Curl is not installed. Please install curl. If curl is installed, check your path and try again"
    fi

}

update_installer() {
    local installer_path=$0
    local remote_hash=$(curl https://git.io/fpcV6 -sSfL | sha512sum | cut -d " " -f 1)
    local local_hash=$(sha512sum $0 | cut -d " " -f 1)

    if [ "$remote_hash" == "$local_hash" ]; then
        error "Installer is up to date"
    else
        if [ -w "$0" ]; then
            curl https://git.io/fpcV6 -sSfL > $installer_path
            local new_hash=$(sha512sum $0 | cut -d " " -f 1)
            if [ "$remote_hash" == "$new_hash" ]; then
                echo "Installer has been updated"
            else
                echo "Cannot update installer"
            fi
        else
            error "Cannot update installer; check permissiosns and try again."
        fi

    fi

}


install_machu_picchu () {
    URL="https://github.com/foundpatterns/mp-installer/archive/master.zip"
    TEMP=temp.zip
    DIR=.mp-installer

    if [ -x "$(command -v curl)" ]; then
        echo Downloading Machu Picchu
        curl -L $URL -o $TEMP
        echo Installing Machu Picchu
        mkdir $DIR
        unzip -q -o temp.zip -d $DIR
        rm $TEMP
        cd $DIR/mp-installer-master
        case $(get_os) in
            Linux | Darwin ) torchbear;;
            * ) torchbear;;
        esac
        STATUS=$?
        cd ../..
        rm -rf $DIR
    else
        error "Curl is not installed. Please install curl. If curl is installed, check your path and try again"
    fi

    if [ $STATUS = "0" ]; then
        echo Machu Picchu installed succesfully
    else
        error Machu Picchu install was unsuccesfull
    fi

    mp refresh
}

install_path() {
    case $(get_os) in
        Linux | Darwin)
            echo "/usr/local/bin"
            ;;
        Android)
            echo "/data/data/com.termux/files/usr/bin"
            ;;
        Windows)
            if [ -d "$CMDER_ROOT" ]; then
                echo "$CMDER_ROOT/bin"
            else
                error Cmder is required to run this installer.
            fi
            ;;
        *)
            error "System is not supported at this time"
            ;;
    esac
}

torchbear_path() {
    case $(get_os) in
        Linux | Darwin | Android)
            echo "$(install_path)/torchbear"
            ;;
        Windows)
            echo "$(install_path)/torchbear.exe"
            ;;
    esac
}

uninstall_torchup() {
    FILE=$(install_path)/torchup
    if [ -f $FILE ]; then
        rm $FILE
    fi
}

uninstall_torchbear() {
    if [ -f "$(torchbear_path)" ]; then
        echo Uninstalling Torchbear.

        rm $(torchbear_path)

        if [ -f "$(torchbear_path)" ]; then
            error Torchbear could not be uninstalled.
        else
            echo Torchbear is now uninstalled.
        fi


        if [ -f $(install_path)/speakeasy ]; then
            rm $(install_path)/speakeasy
        fi

        uninstall_torchup
    else
        error Torchbear is not installed.
    fi
}

uninstall_mp() {
    if [ -f "$(install_path)/mp" ]; then
        echo Uninstalling machu picchu.

        rm $(install_path)/mp
        rm -rf $(install_path)/machu-pichu

        if [ -f "$(install_path)/mp" ]; then
            error Machu Picchu could not be uninstalled.
        else
            echo Machu Picchu is now uninstalled.
        fi
    else
        error Machu Picchu is not installed.
    fi
}

install_torchup() {
    URL=https://raw.githubusercontent.com/foundpatterns/torchbear/master/install.sh
    curl -L $URL -o $1/torchup
    chmod +x $1/torchup
    echo Torchup has been installed
}

install_torchbear() {
    if [ -f "$(torchbear_path)" ]; then
	    local curr_version=($(echo $($(torchbear_path) -V)))
	    local repo_version=$(get_latest_version)

	    if [ "${curr_version[1]}" == "$repo_version" ]; then
            error "Torchbear is up to date."
	    fi
        echo "New version of available"
        echo "Current Version: ${curr_version[1]}"
        echo "Latest Version: $repo_version"
    fi

    echo Downloading torchbear

    DIR=$(install_path)
    download_and_extract $DIR

    if [ -f $DIR/speakeasy ]; then
        rm $DIR/speakeasy
    fi
    ln -Ts $DIR/torchbear $DIR/speakeasy

    if [ -f "$(torchbear_path)" ]; then
	    local version=($(echo $($(torchbear_path) -V)))
        echo Torchbear ${version[1]} has been installed.
    fi

    install_torchup $DIR
}

install_mp() {
    if [ -f "$(torchbear_path)" ]; then

        # Only install mp if not detected
        # TODO: Check for updates for mp
        if [ ! -x "$(command -v mp)" ]; then
            install_machu_picchu
        fi
    else
        error Torchbear is not installed.
    fi
}

error() { echo "$*" 1>&2 ; exit 1; }

case $1 in
    "--install-torchbear")
        install_torchbear
    ;;
    "--install-mp")
        install_mp
    ;;
    "--uninstall")
        uninstall_torchbear
        uninstall_mp
    ;;
    "--uninstall-torchbear")
        uninstall_torchbear
    ;;
    "--uninstall-mp")
        uninstall_mp
    ;;
    "--update")
        update_installer
    ;;
    * )
        install_torchbear
        install_mp
    ;;
esac
