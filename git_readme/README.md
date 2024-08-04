<!-- @format -->

# Git use full command

### Install Git

Linux

```
sudo apt install git-all
```

Check `git` version

```
git --version
```

### Git documentation

```
man git
```

### Porcelain and Plumbing

Git commands are divided into high-level(`Procelain`) commands and
low-level(`plumbing`) commands.

Some porcelain commands

- `git status`
- `git add`
- `git commit`
- `git push`
- `git pull`
- `git log`

Some plumbing commands

- `git apply`
- `git commit-tree`
- `git hash-object`

Git Config

- `git config --get user.name`
- `get config --get user.email`
- `cat ~/.gitconfig` read all gitconfig

Set config

- `git config --add --global user.name "githun_user_name"`
- `git config --add --global user.email "you@gmail.com"`

Edit commit

- `git commit -amend -m "message"` edit commited commit

Read git commit

- `git --no-pager log -n 10` or `git log -n 10` or all `git log`

git commite contents all store in `.git/objects`

- `cat path/to/file`
- `xxd path/to/file` read in hexadecimal

read with git `cat-file`

- `git cat-file -p <hash>`

### Config

Add global configuration

- `git config --add --global user.name "username_git/github"`
- `git config --add --global user.email "user@gmail.com"`

See local config

- `git config --list --local` or `cat .git/config` or
  `git config --list --global`
- `git config --get <section>.<keymap>`
- `git config --unset <section>.<keymap>`
- `git config --remove-section section`

Locations

- System: `/etc/gitconfig`, a file that configure for all git user on the system
- global: `~/.gitconfig`, a file that configures Git for all the projects of a
  user
- Local: `.git/config`, a file that configures Git for a specific project
- worktree: `.git/config.worktree`, a file that configures Git for part of a
  project
