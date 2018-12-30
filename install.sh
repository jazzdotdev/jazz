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
        case $(get_os) in
            Linux | Darwin )
                sudo unzip -o temp.zip -d $1
                ;;
            * )
                unzip -o temp.zip -d $1
                ;;
        esac
        rm temp.zip
    else
        error "Curl is not installed. Please install curl. If curl is installed, check your path and try again"
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
        unzip -q -o temp.zip -d $DIR
        rm $TEMP
        cd $DIR/mp-installer-master
        case $(get_os) in
            Linux | Darwin ) sudo torchbear;;
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
        echo Machu Picchu install was unsuccesfull
    fi
}

torchbear_path() {
    case $(get_os) in
        Linux | Darwin)
            echo "/usr/local/bin/torchbear"
            ;;
        Android)
            echo "/data/data/com.termux/files/usr/bin/torchbear"
            ;;
        Windows)
            if [ -d "$CMDER_ROOT" ]; then
                echo "$CMDER_ROOT/bin/torchbear.exe"
            else
                error Cmder is required to run this installer.
            fi
            ;;
    esac
}

uninstall() {
    if [ -f "$(torchbear_path)" ]; then
        echo Uninstalling torchbear.
        case $(get_os) in
            Linux | Darwin)
                sudo rm $(torchbear_path)
                ;;
            * )
                rm $(torchbear_path)
                ;;
        esac
        if [ -f "$(torchbear_path)" ]; then
            error Torchbear could not be uninstalled.
        else
            echo Torchbear is now uninstalled.
        fi
    else
        error Torchbear is not installed.
    fi
}

install() {
    echo System Type: $(get_os)
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

    case $(get_os) in
        Linux | Darwin)
            download_and_extract "/usr/local/bin"
            ;;
        Android)
            download_and_extract "/data/data/com.termux/files/usr/bin"
            ;;
        Windows)
            download_and_extract "$CMDER_ROOT/bin"
            ;;
        *)
            error "System is not supported at this time"
            ;;
    esac

    if [ -f "$(torchbear_path)" ]; then
	    local version=($(echo $($(torchbear_path) -V)))
        echo Torchbear ${version[1]} has been installed.
    fi

    # Only install mp if not detected
    if [ ! -x "$(command -v mp)" ]; then
        read -p "Do you want to install machu-picchu (y/n)? " choice </dev/tty
        case "$choice" in 
            y|Y )
                install_machu_picchu
                ;;
            n|N )
                # Ignore
                ;;
            * ) echo "Invalid option";;
        esac
    fi

}

error() { echo "$*" 1>&2 ; exit 1; }

if [[ $1 = "--uninstall" ]]; then
    uninstall
else
    install
fi
