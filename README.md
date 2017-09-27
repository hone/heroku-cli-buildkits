# Buildkits2 Plugin

Create and publish buildpacks.

## Install

```
$ heroku plugins:install heroku-cli-buildkits
```

## Usage

Create a new buildpack:

```
$ heroku _buildkits:init my-ruby-buildpack
```

Create a [new Github repo](https://github.com/new) for the project and push your buildpack code to it:

```
$ git push origin master
```

Register your buildpack:

```
$ heroku _buildkits:register johndoe/ruby
```

Create a tagged version and push it to Github:

```
$ git tag v1
$ git push --tags
```

Publish the tag as a Heroku buildpack:

```
$ heroku _buildkits:publish johndoe/ruby v1
```

Use the buildpack on an app:

```
$ cd my-ruby-app
$ heroku create
$ heroku _buildkits:set johndoe/ruby
$ git push heroku master
```
