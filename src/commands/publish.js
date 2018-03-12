import {Command} from 'cli-engine-heroku'
import binary from 'node-pre-gyp'
import path from 'path'
var addonPath = binary.find(path.resolve(path.join(__dirname, '../../package.json')))
var addon = require(addonPath)

export default class Create extends Command {
  static topic = 'buildpacks'
  static command = 'publish'
  static description = 'publish new revision of buildpack'
  static args = [
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
    let nameParts = this.args.name.split('/')
    if (nameParts.length !== 2) {
      this.out.error(`Invalid buildpack name: ${this.args.name}`)
      return
    }
    let namespace = nameParts[0]
    let name = nameParts[1]

    addon.publish(namespace, name, this.args.tag)
  }
}
