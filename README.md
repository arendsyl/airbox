# airbox
Dirty CLI Tool to get mobile data left of an airbox

## Installation :

    $ git clone https://github.com/arendsyl/airbox.git
    $ cd airbox
    $ cargo build --release

## Using it without having to type the path :
    # ln -s `readlink -f target/release/airbox` /usr/local/bin/airbox
    
## Uninstalling :
    $ rm -rf airbox
    # rm /usr/local/bin/airbox
