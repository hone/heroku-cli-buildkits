import {Command, flags} from 'cli-engine-heroku'
import binary from 'node-pre-gyp'
import path from 'path'
var addonPath = binary.find(path.resolve(path.join(__dirname, '../../package.json')))
var addon = require(addonPath)

export default class Create extends Command {
  static topic = '_buildkits'
  static command = 'init'
  static description = 'bootstrap a new buildpack'
  static args = [
    {
      name: 'name',
      optional: false,
      description: 'name of the buildpack'
    }
  ]

  async run () {
    addon.init(this.args.name)
  }
}

