import {Command} from 'cli-engine-heroku'
import binary from 'node-pre-gyp'
import path from 'path'
var addonPath = binary.find(path.resolve(path.join(__dirname, '../../package.json')))
var addon = require(addonPath)

export default class Create extends Command {
  static topic = '_buildkits'
  static command = 'search'
  static description = 'search for buildpacks'
  static args = [
    {
      name: 'name',
      optional: true,
      description: 'name of the buildpack'
    }
  ]

  async run () {
    addon.search(this.args.name)
  }
}
