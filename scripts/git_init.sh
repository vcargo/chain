#!/usr/bin/env sh

# Adding your SSH key to the ssh-agent
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# Setting up Git
git_name="vcargo"
git_email="158460728+vcargo@users.noreply.github.com"
if [ -f /.dockerenv ]; then
  git config --global core.editor vim
  git config --global init.defaultBranch main
  git config --global user.name "$git_name"
  git config --global user.email "$git_email"
else
  git config --local user.name "$git_name"
  git config --local user.email "$git_email"
fi
