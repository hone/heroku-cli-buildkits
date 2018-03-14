'use strict'

let co = require('co')
let cli = require('heroku-cli-util')
let BuildpackCommand = require('../buildpacks.js')

function * run (context, heroku) {
  let buildpackCommand = new BuildpackCommand(context, heroku, 'set', 'set')

  buildpackCommand.validateUrlPassed()

  let buildpacksGet = yield buildpackCommand.get()

  buildpackCommand.validateUrlNotSet(buildpacksGet)

  var spliceIndex
  if (buildpackCommand.index === null) {
    spliceIndex = 0
  } else {
    let foundIndex = buildpackCommand.findIndex(buildpacksGet, buildpackCommand.index)
    spliceIndex = (foundIndex === -1) ? buildpacksGet.length : foundIndex
  }

  yield buildpackCommand.mutate(buildpacksGet, spliceIndex)
}

module.exports = {
  default: {
    topic: 'buildpacks',
      command: 'set',
      args: [
      {name: 'url', optional: false, description: 'namespace/name of the buildpack in the registry or URL of the buildpack'}
    ],
      flags: [
      {name: 'index', char: 'i', hasValue: true, description: 'the 1-based index of the URL in the list of URLs'}
    ],
      description: 'set new app buildpack, overwriting into list of buildpacks if necessary',
      help: `Example:

       $ heroku buildpacks:set -i 1 heroku/ruby
  `,
      needsApp: true,
      needsAuth: true,
      run: cli.command(co.wrap(run))
  }
}
