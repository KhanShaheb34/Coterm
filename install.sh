# install rust if not already installed
if ! command -v rustc &> /dev/null
then
    echo "Rust is not installed!"
    echo "Installing rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "Done!"
fi

# go to the home directory
cd $HOME

# clone the repo to the current directory
echo "Cloning the repo..."
git clone https://github.com/KhanShaheb34/coterm.git

# change directory to the repo
cd coterm

# install the binary
echo "Installing the binary..."
cargo build --release

# copy the binary to PATH
echo "Copying the binary to PATH..."
sudo cp ./target/release/coterm /usr/local/bin/ct
sudo cp ./target/release/coterm /usr/local/bin/coterm

# change directory to the home directory
cd $HOME

# remove the repo
echo "Removing the repo..."
rm -rf coterm

echo "Done!"
# done