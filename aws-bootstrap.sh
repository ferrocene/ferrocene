mkdir ~/.tmp && cd $_

# install aws-cli v2
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
  unzip awscliv2.zip && \
  sudo ./aws/install

mkdir ~/.bin && cd $_
wget https://raw.githubusercontent.com/pahud/vscode/main/.devcontainer/bin/aws-sso-credential-process && \
chmod +x aws-sso-credential-process


aws configure set credential_process ${HOME}/.bin/aws-sso-credential-process
touch ~/.aws/credentials && chmod 600 $_
aws configure sso --profile default