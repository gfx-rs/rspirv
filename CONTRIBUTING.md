# Contributing to rspirv

It's great to know that you are looking to contribute to rspirv! To keep the
project in a consistent state on various aspects, we have the following
conventions that it would be very nice if you can consider:

## Code review

* Please address comments with a new commit instead of force pushing to
  the branch. (This makes it quite easy to see what's changed for addressing
  the comments. Otherwise the reviewer needs to re-read the whole pull request
  again. That delays review process.)
* Code will be squash merged after approval to leave only one commit in the
  history and commit messages may be edited accordingly. (The interactive
  comment addressing procedure is not particularly interesting from the
  project history's perspective. It obscures the Git history and makes
  reverting a bad commit hard.)

## Pull request

* A pull request should focus on just one task.
* A pull request doing refactoring (esp. for moving files around) should not
  be coupled with functionality changes.
* A smaller pull request is preferable.

The above conventions make it easier to read and review a pull request.

## Commit message

* Commit messages should start with a concise oneliner. (So that it can
  render nicely in GitHub UI and CLI short log form.)
* Commit messages should have detailed explanation of the changes following
  the oneliner and an empty line. (So that by reading just the commit message
  we know what's changed without going to the code.)
* The detailed explanation should break into lines properly. (So that when
  using CLI to read the history, we won't see a very long sentence that
  got truncated, esp. inside a terminal multiplexer where screen real estate
  is limited.)

The above conventions helps us to have a clean and healthy Git history.

## Code change

* Code adding new functionalities should have tests.
* Code adding new functionalities should have documentation for public APIs.

## Licensing

Copyrights are retained by their contributors, no copyright assignment is
required to contribute to rspirv.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be licensed under the Apache License, Version 2.0, see ([LICENSE](LICENSE) or
http://www.apache.org/licenses/LICENSE-2.0), without any additional terms or
conditions.
