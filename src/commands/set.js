import {Command, flags} from 'cli-engine-heroku'
import binary from 'node-pre-gyp'
import path from 'path'
var addonPath = binary.find(path.resolve(path.join(__dirname, '../../package.json')))
var addon = require(addonPath)

export default class Create extends Command {
  static topic = '_buildkits'
  static command = 'set'
  static description = 'bootstrap a new buildpack'
  static flags = {
    app: flags.app({required: true})
  }
  static args = [
    {
      name: 'name',
      optional: false,
      description: 'name of the buildpack'
    }
  ]

  async run () {
    child.execSync(`heroku buildpacks:set https://heroku-buildkits-production.s3.amazonaws.com/buildpacks/${this.args.name}.tgz --app ${this.flags.app}`, {stdio: 'inherit'})
  }
}
