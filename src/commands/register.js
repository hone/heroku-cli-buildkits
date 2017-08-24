import {Command, flags} from 'cli-engine-heroku'
import binary from 'node-pre-gyp'
import path from 'path'
var addonPath = binary.find(path.resolve(path.join(__dirname, '../../package.json')))
var addon = require(addonPath)

export default class Create extends Command {
  static topic = '_buildkits'
  static command = 'register'
  static description = 'create a buildpack'
  static args = [
    {
      name: 'repo',
      optional: false,
      description: 'git repo for the buildpack'
    },
    {
      name: 'namespace',
      optional: false,
      description: 'namespace the buildpack belongs in'
    },
    {
      name: 'name',
      optional: false,
      description: 'name of the buildpack'
    }
  ]

  async run () {
    addon.register(this.args.repo, this.args.namespace, this.args.name)
  }
}
