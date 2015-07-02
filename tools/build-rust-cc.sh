echo "-------------------------------"
echo "setup linux env"
echo "-------------------------------"
sudo apt-get update
sudo apt-get -y install apt-utils git gcc g++ perl python2.7 curl make unzip nano
 
echo "-------------------------------"
echo "download rpi toolchain"
echo "-------------------------------"
wget https://github.com/raspberrypi/tools/archive/master.zip
unzip master.zip
export PATH=$PWD/tools-master/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin:$PATH
 
echo "-------------------------------"
echo "First steps"
echo "-------------------------------"
mkdir -p $HOME/toolchains/src
cd $HOME/toolchains/src
git clone https://github.com/mozilla/rust.git
cd rust
git submodule update --init
 
echo "-------------------------------"
echo "Configuring Rust"
echo "-------------------------------"
cd $HOME/toolchains/src/rust
mkdir build
cd build
mkdir -p $HOME/toolchains/var/lib
mkdir $HOME/toolchains/etc
 
$PWD/../configure --prefix=$HOME/toolchains                       \
    --host=x86_64-unknown-linux-gnu --disable-llvm-assertions     \
    --target=x86_64-unknown-linux-gnu,arm-unknown-linux-gnueabihf \
    --localstatedir=$HOME/toolchains/var/lib                      \
    --sysconfdir=$HOME/toolchains/etc
cd x86_64-unknown-linux-gnu
find . -type d -exec mkdir -p ../arm-unknown-linux-gnueabihf/\{\} \;
 
echo "-------------------------------"
echo "Building cross LLVM"
echo "-------------------------------"
cd $HOME/toolchains/src/rust/build/x86_64-unknown-linux-gnu/llvm
$HOME/toolchains/src/rust/src/llvm/configure --enable-target=x86,x86_64,arm,mips  \
    --enable-optimized --disable-assertions --disable-docs --enable-bindings=none \
    --disable-terminfo --disable-zlib --disable-libffi                            \
    --with-python=/usr/bin/python2.7
make -j$(nproc)
 
cd $HOME/toolchains/src/rust/build/arm-unknown-linux-gnueabihf/llvm
$HOME/toolchains/src/rust/src/llvm/configure --enable-target=x86,x86_64,arm,mips  \
    --enable-optimized --disable-assertions --disable-docs --enable-bindings=none \
    --disable-terminfo --disable-zlib --disable-libffi                            \
    --with-python=/usr/bin/python2.7 --build=x86_64-unknown-linux-gnu             \
    --host=arm-linux-gnueabihf --target=arm-linux-gnueabihf
make -j$(nproc)
 
echo "-------------------------------"
echo "Enable llvm-config for the cross LLVM build"
echo "-------------------------------"
 
cd $HOME/toolchains/src/rust/build/arm-unknown-linux-gnueabihf/llvm/Release/bin
mv llvm-config llvm-config-arm
ln -s ../../BuildTools/Release/bin/llvm-config .
# (Now test to be sure this works.)
./llvm-config --cxxflags
# (You should see some CXX flags printed out here!)
 
echo "-------------------------------"
echo "Making RBS use our LLVM builds"
echo "-------------------------------"
cd $HOME/toolchains/src/rust/build/
chmod 0644 config.mk
grep 'CFG_LLVM_[BI]' config.mk |                                          \
    sed 's/x86_64\(.\)unknown.linux.gnu/arm\1unknown\1linux\1gnueabihf/g' \
    >> config.mk
 
cd $HOME/toolchains/src/rust
sed -i.bak 's/\([\t]*\)\(.*\$(MAKE).*\)/\1#\2/' mk/llvm.mk    
 
echo "-------------------------------"
echo "Building a working librustc for the cross architecture"
echo "-------------------------------"
cd $HOME/toolchains/src/rust
sed -i.bak                                                                         \
    's/^CRATES := .*/TARGET_CRATES += $(HOST_CRATES)\nCRATES := $(TARGET_CRATES)/' \
    mk/crates.mk
sed -i.bak                                                                                        \
    's/\(.*call DEF_LLVM_VARS.*\)/\1\n$(eval $(call DEF_LLVM_VARS,arm-unknown-linux-gnueabihf))/' \
    mk/main.mk
sed -i.bak 's/foreach host,$(CFG_HOST)/foreach host,$(CFG_TARGET)/' mk/rustllvm.mk
 
cd $HOME/toolchains/src/rust
sed -i.bak 's/.*target_arch = .*//' src/etc/mklldeps.py
 
cd $HOME/toolchains/src/rust/build
arm-unknown-linux-gnueabihf/llvm/Release/bin/llvm-config --libs \
    | tr '-' '\n' | sort > arm
x86_64-unknown-linux-gnu/llvm/Release/bin/llvm-config --libs \
    | tr '-' '\n' | sort > x86
diff arm x86
 
echo "-------------------------------"
echo "Build it, part 1"
echo "-------------------------------"
cd $HOME/toolchains/src/rust/build
make -j$(nproc)
 
echo "-------------------------------"
echo "Build it, part 2"
echo "-------------------------------"
cd $HOME/toolchains/src/rust/build
LD_LIBRARY_PATH=$PWD/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnueabihf/lib:$LD_LIBRARY_PATH    \
    ./x86_64-unknown-linux-gnu/stage2/bin/rustc --cfg stage2 -O --cfg rtopt                                         \
    -C linker=arm-linux-gnueabihf-g++ -C ar=arm-linux-gnueabihf-ar                                  \
    -C target-feature=+vfp2,-neon  -C target-cpu=arm1176jzf-s                               \
    --cfg debug -C prefer-dynamic --target=arm-unknown-linux-gnueabihf                                              \
    -o x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/bin/rustc --cfg rustc                \
    $PWD/../src/driver/driver.rs
LD_LIBRARY_PATH=$PWD/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnueabihf/lib:$LD_LIBRARY_PATH    \
    ./x86_64-unknown-linux-gnu/stage2/bin/rustc --cfg stage2 -O --cfg rtopt                                         \
    -C linker=arm-linux-gnueabihf-g++ -C ar=arm-linux-gnueabihf-ar                                  \
    -C target-feature=+vfp2,-neon  -C target-cpu=arm1176jzf-s                               \
    --cfg debug -C prefer-dynamic --target=arm-unknown-linux-gnueabihf                                              \
    -o x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/bin/rustdoc --cfg rustdoc            \
    $PWD/../src/driver/driver.rs
 
echo "-------------------------------"
echo "Package"
echo "-------------------------------"
cd $HOME/toolchains/src/rust/build/
mkdir -p cross-dist/lib/rustlib/arm-unknown-linux-gnueabihf
cd cross-dist
cp -R ../x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/* \
    lib/rustlib/arm-unknown-linux-gnueabihf
mv lib/rustlib/arm-unknown-linux-gnueabihf/bin .
cd lib
for i in rustlib/arm-unknown-linux-gnueabihf/lib/*.so; do ln -s $i .; done
cd ../
tar cjf ../rust_arm-unknown-linux-gnueabihf_dist.tbz2 .