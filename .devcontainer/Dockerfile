# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.234.0/containers/ubuntu/.devcontainer/base.Dockerfile

# [Choice] Ubuntu version (use ubuntu-22.04 or ubuntu-18.04 on local arm64/Apple Silicon): ubuntu-22.04, ubuntu-20.04, ubuntu-18.04
ARG VARIANT="jammy"
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
RUN apt update
RUN apt install -y curl
RUN curl https://packages.microsoft.com/config/ubuntu/22.04/packages-microsoft-prod.deb > /tmp/packages-microsoft-prod.deb
RUN dpkg -i /tmp/packages-microsoft-prod.deb
RUN echo 'Package: *\n\
Pin: origin "packages.microsoft.com"\n\
Pin-Priority: 1001'\
> /etc/apt/preferences.d/99microsoft-dotnet.pref
RUN apt update
RUN apt install -y powershell
RUN apt install -y cmake
RUN apt install -y build-essential
RUN apt install -y liblttng-ust-dev
RUN apt install -y lttng-tools
RUN apt install -y linux-tools-`uname -r`
RUN apt install -y dotnet-sdk-6.0
