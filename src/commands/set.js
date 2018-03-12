import {Command, flags} from 'cli-engine-heroku'
import child from 'child_process'

export default class Create extends Command {
  static topic = 'buildpacks'
  static command = 'set'
  static description = 'bootstrap a new buildpack'
  static flags = {
    app: flags.app({required: true}),
    index: flags.number({char: 'i'})
  }
  static args = [
    {
      name: 'name',
      optional: false,
      description: 'name of the buildpack'
    }
  ]

  async run () {
    var opts = ""
    if (this.flags.index) {
      opts = `--index ${this.flags.index}`
    }
    child.execSync(`heroku buildpacks:set ${opts} https://heroku-buildkits-production.s3.amazonaws.com/buildpacks/${this.args.name}.tgz --app ${this.flags.app}`, {stdio: 'inherit'})
  }
}
