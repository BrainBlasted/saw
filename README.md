# Simple Apt Wrapper (saw)

A seriously simple wrapper around apt for Debian based systems.

# Features?

You never have to worry about this:

```shell
$ apt install foo
E: Could not open lock file /var/lib/dpkg/lock - open (13: Permission denied)
E: Unable to lock the administration directory (/var/lib/dpkg/), are you root?

$ sudo apt install foo
```

Saw automatically handles calling `sudo`.

# Is That It?

Yup.

```
Copyright (C) 2017  Christopher Davis

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
```
