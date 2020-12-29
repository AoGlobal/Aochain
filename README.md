* [Introduction](#Introduction)
* [Design Concept](#Design Concept)
* [Road Map](#Road Map)




# Introduction

![AOCHAIN](https://wiki.aochain.io/assets/uploads/Polkadot-Releases-Major-Milestone-For-Its-Substrate-Blockchain-Framework.jpg)

##### AoChain is an active supporter of the Polkadot ecology, committed to enabling everyone to use Web3.0 blockchain technology efficiently and at low cost, and enjoy the convenient services and ecological dividends brought by the Web3.0 information technology era. Using the Aochain blockchain application, you can build DEFI smart contract products, DGAME smart contract game products, Erc20 standard Token issuance, Dapp development and NFT asset release in one-stop.

#### AoChain will actively participate in various applications of the Polkadot ecology, and plans to apply for validator nodes and participate in the auction of parachains to further integrate the Polkadot ecology.



--

# Design Concept


#### True interoperability：

Polkadot enables cross-blockchain transfers of any type of data or asset, not just tokens. Connecting to Polkadot gives you the ability to interoperate with a wide variety of blockchains in the Polkadot network.

#### Economic & transactional scalability：

Polkadot provides unprecedented economic scalability by enabling a common set of validators to secure multiple blockchains. Polkadot provides transactional scalability by spreading transactions across multiple parallel blockchains.


#### Easy blockchain innovation：

Create a custom blockchain in minutes using the Substrate framework. Connect your chain to Polkadot and get interoperability and security from day one. This ease of development helps Polkadot's network grow.

#### Forkless and future-proof：

Polkadot can upgrade without hard forks to integrate new features or fix bugs. This capability enables Polkadot to easily adapt to changes and upgrade itself as better technologies become available.

#### Security for everyone:

Polkadot's novel data availability and validity scheme allows chains to interact with each other in a meaningful way. Chains remain independent in their governance, but united in their security.

#### User-driven network governance:

Polkadot has a sophisticated governance system where all stakeholders have a voice. Upgrades to the network are coordinated on-chain and enacted autonomously, ensuring that Polkadot’s development reflects the values of the community and avoids stagnation.


#### Governance:

Polkadot token holders have complete control over the protocol. All privileges, which on other platforms are exclusive to miners, will be given to the Relay Chain participants (DOT holders), including managing exceptional events such as protocol upgrades and fixes.


#### Staking:

Game theory incentivizes token holders to behave in honest ways. Good actors are rewarded by this mechanism whilst bad actors will lose their stake in the network. This ensures the network stays secure.


Bonding:
New parachains are added by bonding tokens. Outdated or non-useful parachains are removed by removing bonded tokens. This is a form of proof of stake.


![](https://wiki.aochain.io/assets/uploads/WechatIMG1565.png)



# Road Map

* For more details, go to 

  [AOCHAIN](http://aochain.io)

  [WIKI](https://wiki.aochain.io)





# Installation

Since Aochain is built with [the Rust programming language](https://www.rust-lang.org/), the first thing you will need to do is prepare the computer for Rust development - these steps will vary based on the computer's operating system. Once Rust is configured, you will use its toolchains to interact with Rust projects; the commands for Rust's toolchains will be the same for all supported, Unix-based operating systems.

## Unix-Based Operating Systems

### macOS

Open the Terminal application and execute the following commands:

```bash
# Install Homebrew if necessary https://brew.sh/
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"

# Make sure Homebrew is up-to-date, install openssl and cmake
brew update
brew install openssl cmake
```

### Ubuntu/Debian

Use a terminal shell to execute the following commands:

```bash
sudo apt update
# May prompt for location information
sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev curl

```

### Arch Linux

Run these commands from a terminal:

```bash
pacman -Syu --needed --noconfirm cmake gcc openssl-1.0 pkgconf git clang
export OPENSSL_LIB_DIR="/usr/lib/openssl-1.0"
export OPENSSL_INCLUDE_DIR="/usr/include/openssl-1.0"
```

## Windows

If you are trying to set up a Windows computer to build Aochain, do the following:

1. Download and install "Build Tools for Visual Studio:"

   - You can get it at this link: https://aka.ms/buildtools.
   - Run the installation file: `vs_buildtools.exe`.
   - Ensure the "Windows 10 SDK" component is included when installing the Visual C++ Build Tools.
   - Restart your computer.

2. Install Rust:

   - Detailed instructions are provided by the [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows).

     - Download from: https://www.rust-lang.org/tools/install.

     - Run the installation file: `rustup-init.exe`.

       > Note that it should **not** prompt you to install `vs_buildtools` since you did it in step 1.

     - Choose "Default Installation."

     - To get started, you need Cargo's bin directory (`%USERPROFILE%\.cargo\bin`) in your PATH environment variable. Future applications will automatically have the correct environment, but you may need to restart your current shell.

3. Run these commands in Command Prompt (`CMD`) to set up your Wasm Build Environment:

   ```bash
   rustup update nightly
   rustup update stable
   rustup target add wasm32-unknown-unknown --toolchain nightly
   
   ```

4. Install LLVM: https://releases.llvm.org/download.html

5. Install OpenSSL with `vcpkg`:

   ```bash
   mkdir C:\Tools
   cd C:\Tools
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   .\bootstrap-vcpkg.bat
   .\vcpkg.exe install openssl:x64-windows-static
   
   ```

6. Add OpenSSL to your System Variables using PowerShell:

   ```powershell
   $env:OPENSSL_DIR = 'C:\Tools\vcpkg\installed\x64-windows-static'
   $env:OPENSSL_STATIC = 'Yes'
   [System.Environment]::SetEnvironmentVariable('OPENSSL_DIR', $env:OPENSSL_DIR, [System.EnvironmentVariableTarget]::User)
   [System.Environment]::SetEnvironmentVariable('OPENSSL_STATIC', $env:OPENSSL_STATIC, [System.EnvironmentVariableTarget]::User)
   
   ```

7. Finally, install `cmake`: https://cmake.org/download/

## Rust Developer Environment

This guide uses [`rustup`](https://rustup.rs/) to help manage the Rust toolchain. First install and configure `rustup`:

```bash
# Install
curl https://sh.rustup.rs -sSf | sh
# Configure
source ~/.cargo/env
```

Configure the Rust toolchain to default to the latest stable version:

```bash
rustup default stable
```

### WebAssembly Compilation

Aochain uses [WebAssembly](https://webassembly.org/) (Wasm) to produce portable blockchain runtimes. You will need to configure your Rust compiler to use [`nightly` builds](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) to allow you to compile Aochain runtime code to the Wasm target.

#### Rust Nightly Toolchain

Developers building with Aochain should use a specific Rust nightly version that is known to be compatible with the version of Substrate they are using; this version will vary from project to project and different projects may use different mechanisms to communicate this version to developers. For instance, the Polkadot client specifies this information in its [release notes](https://github.com/paritytech/polkadot/releases). The Substrate Node Template uses an [init script](https://github.com/substrate-developer-hub/substrate-node-template/blob/master/scripts/init.sh) and [Makefile](https://github.com/substrate-developer-hub/substrate-node-template/blob/master/Makefile) to specify the Rust nightly version and encapsulate the following steps. Use Rustup to install the correct nightly:

```bash
rustup install nightly-<yyyy-MM-dd>
```

#### Wasm Toolchain

Now, configure the nightly version to work with the Wasm compilation target:

```bash
rustup target add wasm32-unknown-unknown --toolchain nightly-<yyyy-MM-dd>

```

#### Specifying Nightly Version

Use the `WASM_BUILD_TOOLCHAIN` environment variable to specify the Rust nightly version a Substrate project should use for Wasm compilation:

```bash
WASM_BUILD_TOOLCHAIN=nightly-<yyyy-MM-dd> cargo build --release
```

Note that this only builds *the runtime* with the specified nightly. The rest of project will be compiled with the default toolchain, i.e. the latest installed stable toolchain.

#### Latest Nightly for Substrate `master`

Developers that are building Substrate *itself* should always use the latest bug-free versions of Rust stable and nightly. This is because the Substrate codebase follows the tip of Rust nightly, which means that changes in Substrate often depend on upstream changes in the Rust nightly compiler. To ensure your Rust compiler is always up to date, you should run:

```bash
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

**It may be necessary to occasionally rerun `rustup update`** if a change in the upstream Substrate codebase depends on a new feature of the Rust compiler.

#### Downgrading Rust Nightly

If your computer is configured to use the latest Rust nightly and you would like to downgrade to a specific nightly version, follow these steps(Aochain should use nightly-2020-10-05):

```sh
rustup uninstall nightly
rustup install nightly-2020-10-05
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-05
```

# Compiling Aochain

1. Clone the Aochain Source Code

   ```bash
   git clone https://github.com/AoGlobal/Aochain.git
   ```

2. Compile Source Code

   ```bash
   cargo build --release
   ```

   

# Start Your Node

Run the following commands to start your node:

```bash
./target/release/gama --base-path <YourDataDir> --name <YourNodeName>
```

> more tutorials refer to [substrate](https://substrate.dev/en/tutorials)
