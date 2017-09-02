import {Command, flags} from 'cli-engine-heroku'
import binary from 'node-pre-gyp'
import path from 'path'
var addonPath = binary.find(path.resolve(path.join(__dirname, '../../package.json')))
var addon = require(addonPath)

export default class Create extends Command {
  static topic = '_buildkits'
  static command = 'publish'
  static description = 'publish new revision of buildpack'
  static args = [
    {
      name: 'namespace',
      optional: false,
      description: 'namespace of the buildpack'
    },
    {
      name: 'name',
      optional: false,
      description: 'name of the buildpack'
    },
    {
      name: 'tag',
      optional: false,
      description: 'tag to publish'
    }
  ]

  async run () {
    addon.publish(this.args.namespace, this.args.name, this.args.tag)
  }
}


